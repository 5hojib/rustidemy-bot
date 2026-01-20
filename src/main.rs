use anyhow::Result;

mod config;
use config::Config;

mod rss_tracker;
use rss_tracker::RssFeedTracker;

mod udemy_extractor;

mod web;
use web::server;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;

    let web_server = tokio::spawn(async {
        server::start_server().await;
    });

    let rss_tracker = tokio::spawn(async move {
        let tracker = RssFeedTracker::new(&config).await?;
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
    }

    Ok(())
}
