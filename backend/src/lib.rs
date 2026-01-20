
use axum::{
    routing::{get, get_service},
    Router, Json, http::StatusCode,
};
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;
use rss::Channel;
use scraper::{Html, Selector};
use chrono::{DateTime, Utc};
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use http_body_util::BodyExt;
use bytes::Buf;
use hyper::body::Incoming;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumbnail: String,
    pub link: String,
    pub category: Option<String>,
    #[serde(rename = "highResCover")]
    pub high_res_cover: Option<String>,
    #[serde(rename = "resolvedUdemyUrl")]
    pub resolved_udemy_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

pub type Cache = Arc<Mutex<Vec<Course>>>;
type HttpClient = Client<HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>, http_body_util::Full<bytes::Bytes>>;

const RSS_URL: &str = "https://www.discudemy.com/feed";
const CACHE_DURATION_MINUTES: u64 = 10;
const CACHE_EXPIRATION_DAYS: i64 = 2;

pub async fn run_server() {
    let cache: Cache = Arc::new(Mutex::new(Vec::new()));
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build(https);

    let initial_cache_clone = cache.clone();
    let client_clone = client.clone();
    tokio::spawn(async move {
        if let Err(e) = fetch_and_update_cache(initial_cache_clone, client_clone, RSS_URL).await {
            eprintln!("Initial cache update failed: {}", e);
        }
    });

    let recurring_cache_clone = cache.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(CACHE_DURATION_MINUTES * 60)).await;
            println!("Updating cache...");
            if let Err(e) = fetch_and_update_cache(recurring_cache_clone.clone(), client.clone(), RSS_URL).await {
                eprintln!("Scheduled cache update failed: {}", e);
            }
        }
    });

    let app = Router::new()
        .nest_service("/", get_service(ServeDir::new("dist")))
        .route("/api/courses", get(get_courses));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.with_state(cache)).await.unwrap();
}

async fn get_courses(axum::extract::State(cache): axum::extract::State<Cache>) -> (StatusCode, Json<Vec<Course>>) {
    let courses = cache.lock().expect("Mutex lock should not be poisoned").clone();
    (StatusCode::OK, Json(courses))
}

async fn robust_fetch(client: &HttpClient, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let uri = url.parse()?;
    let res = client.get(uri).await?;
    let body_bytes = res.into_body().collect().await?.to_bytes();
    Ok(String::from_utf8(body_bytes.to_vec())?)
}

async fn resolve_udemy_data(client: &HttpClient, initial_url: &str) -> (Option<String>, Option<String>) {
    let meta_cover = {
        let article_html = match robust_fetch(client, initial_url).await {
            Ok(html) => html,
            Err(_) => return (None, None),
        };
        let article_doc = Html::parse_document(&article_html);
        let cover_selector = Selector::parse("meta[property='og:image']").unwrap();
        article_doc
            .select(&cover_selector)
            .next()
            .and_then(|e| e.value().attr("content"))
            .map(String::from)
    };

    let udemy_link = {
        let url_parts: Vec<&str> = initial_url.split('/').filter(|s| !s.is_empty()).collect();
        if let Some(slug) = url_parts.last() {
            let go_url = format!("https://www.discudemy.com/go/{}", slug);
            if let Ok(go_html) = robust_fetch(client, &go_url).await {
                let go_doc = Html::parse_document(&go_html);
                let link_selector = Selector::parse("a[href]").unwrap();
                go_doc
                    .select(&link_selector)
                    .find(|e| {
                        e.value()
                            .attr("href")
                            .unwrap_or("")
                            .contains("udemy.com")
                    })
                    .and_then(|e| e.value().attr("href").map(String::from))
            } else {
                None
            }
        } else {
            None
        }
    };

    (udemy_link, meta_cover)
}

pub async fn fetch_and_update_cache(cache: Cache, client: HttpClient, rss_url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting cache update process...");

    let rss_content = robust_fetch(&client, rss_url).await?;
    let channel = Channel::read_from(rss_content.as_bytes())?;

    let mut new_courses = Vec::new();

    for item in channel.items().iter().take(50) {
        if let Some(link) = item.link() {
            let (resolved_url, cover) = resolve_udemy_data(&client, link).await;

            let course = Course {
                id: link.to_string(),
                title: item.title().unwrap_or("No Title").to_string(),
                description: item.description().unwrap_or("").to_string(),
                thumbnail: cover.clone().unwrap_or_default(),
                link: link.to_string(),
                category: item.categories().first().map(|c| c.name().to_string()),
                high_res_cover: cover,
                resolved_udemy_url: resolved_url,
                created_at: Utc::now(),
            };
            new_courses.push(course);
        }
    }

    let mut courses = cache.lock().expect("Mutex lock should not be poisoned");

    // Prune old courses
    let now = Utc::now();
    courses.retain(|course| {
        now.signed_duration_since(course.created_at).num_days() < CACHE_EXPIRATION_DAYS
    });

    // Add new courses and ensure uniqueness
    for new_course in new_courses {
        if !courses.iter().any(|c| c.link == new_course.link) {
            courses.push(new_course);
        }
    }

    courses.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    println!("Cache update complete. Total courses: {}", courses.len());

    Ok(())
}
