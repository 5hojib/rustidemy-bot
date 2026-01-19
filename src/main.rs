use anyhow::{Context, Result};
use rss::Channel;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

mod config;
use config::Config;

mod rss_tracker;
use rss_tracker::RssFeedTracker;

mod udemy_extractor;
use udemy_extractor::{extract_main_description, extract_thumbnail_url, extract_udemy_url};

mod web;
use web::server;

mod web_cache;
use web_cache::{Course, WebCache};

async fn update_web_cache_task(config: Arc<Config>, cache: Arc<Mutex<WebCache>>) {
    println!(
        "Web cache updater task started. Tracking feed: {}",
        &config.feed_url
    );

    // Initial update on startup
    if let Err(e) = update_cache(&config.feed_url, Arc::clone(&cache)).await {
        eprintln!("Initial web cache update failed: {}", e);
    }

    let mut interval = time::interval(Duration::from_secs(10 * 60)); // 10 minutes
    loop {
        interval.tick().await;
        println!("Updating web cache...");
        if let Err(e) = update_cache(&config.feed_url, Arc::clone(&cache)).await {
            eprintln!("Error updating web cache: {}", e);
        }
    }
}

async fn update_cache(feed_url: &str, cache: Arc<Mutex<WebCache>>) -> Result<()> {
    let response = reqwest::get(feed_url).await?.bytes().await?;
    let channel =
        Channel::read_from(&response[..]).context("Failed to parse RSS feed for web cache")?;

    let mut cache_locked = cache.lock().await;

    for item in channel.items() {
        let title = item.title().unwrap_or("Untitled Course").to_string();
        let raw_description = item.description().unwrap_or("").to_string();
        let body = extract_main_description(&raw_description, &title);

        if let Some(link) = item.link() {
            match extract_udemy_url(link).await {
                Ok(udemy_url) => {
                    let thumbnail_url = extract_thumbnail_url(&raw_description)
                        .unwrap_or_else(|| "https://placehold.co/600x400".to_string());
                    let course = Course {
                        title: title.clone(),
                        description: body.clone(),
                        url: udemy_url.to_string(),
                        thumbnail: thumbnail_url,
                    };
                    cache_locked.add(course);
                }
                Err(e) => {
                    eprintln!("Failed to extract Udemy URL for web cache: {}", e);
                }
            }
        }
    }

    cache_locked.prune();
    if let Err(e) = cache_locked.save() {
        eprintln!("Failed to save web cache: {}", e);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Config::new()?);

    let web_cache = Arc::new(Mutex::new(WebCache::new("web_cache.json")));

    let web_server = tokio::spawn(server::start_server(web_cache.clone()));

    let cache_updater = tokio::spawn(update_web_cache_task(
        Arc::clone(&config),
        Arc::clone(&web_cache),
    ));

    let rss_tracker_config = Arc::clone(&config);
    let rss_tracker = tokio::spawn(async move {
        let tracker = RssFeedTracker::new(&rss_tracker_config).await?;
        tracker.start_tracking(5).await
    });

    tokio::select! {
        web_result = web_server => {
            println!("Web server task completed");
            web_result.map_err(|e| anyhow::anyhow!("Web server error: {}", e))?;
        }
        rss_result = rss_tracker => {
            println!("RSS tracker task completed");
            match rss_result {
                Ok(inner_result) => inner_result?,
                Err(e) => return Err(anyhow::anyhow!("RSS tracker error: {}", e)),
            }
        }
        cache_result = cache_updater => {
            println!("Cache updater task completed");
            cache_result.map_err(|e| anyhow::anyhow!("Cache updater error: {}", e))?;
        }
    }

    Ok(())
}
