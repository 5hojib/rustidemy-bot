
use std::sync::{Arc, Mutex};
use chrono::{Utc, Duration};
use backend::{Course, fetch_and_update_cache};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};

#[tokio::test]
async fn test_cache_pruning_and_fetching() {
    // Arrange
    let server = MockServer::start().await;
    let course_url = format!("{}/course/new", server.uri());

    let rss_feed = format!(r#"
        <rss version="2.0">
            <channel>
                <item>
                    <title>New Course</title>
                    <link>{}</link>
                    <description>New course description</description>
                </item>
            </channel>
        </rss>
    "#, course_url);

    Mock::given(method("GET"))
        .and(path("/feed"))
        .respond_with(ResponseTemplate::new(200).set_body_string(rss_feed))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/course/new"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"<meta property="og:image" content="new_thumbnail">"#))
        .mount(&server)
        .await;

    let cache = Arc::new(Mutex::new(vec![
        Course {
            id: "1".to_string(),
            title: "Old Course".to_string(),
            description: "".to_string(),
            thumbnail: "".to_string(),
            link: "old_link".to_string(),
            category: None,
            high_res_cover: None,
            resolved_udemy_url: None,
            created_at: Utc::now() - Duration::days(3),
        }
    ]));

    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build(https);
    let test_rss_url = format!("{}/feed", server.uri());

    // Act
    fetch_and_update_cache(cache.clone(), client, &test_rss_url).await.unwrap();

    // Assert
    let courses = cache.lock().unwrap();
    assert_eq!(courses.len(), 1);
    assert_eq!(courses[0].title, "New Course");
    assert!(!courses.iter().any(|c| c.title == "Old Course"));
}
