use crate::config::Config;
use crate::udemy_extractor::extract_udemy_url;
use anyhow::{Context, Result};
use rss::Channel;
use std::sync::Arc;
use teloxide::{
    prelude::*,
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, InputFile},
};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use lru::LruCache;

pub struct RssFeedTracker {
    bot: Bot,
    channel_id: ChatId,
    feed_url: String,
    seen_entries: Arc<Mutex<LruCache<String, ()>>>,
}

impl RssFeedTracker {
    pub async fn new(config: &Config) -> Result<Self> {
        let feed_url = "https://www.discudemy.com/feed".to_string();
        Ok(RssFeedTracker {
            bot: Bot::new(config.bot_token.clone()),
            channel_id: config.channel_id,
            feed_url,
            seen_entries: Arc::new(Mutex::new(LruCache::new(500))),
        })
    }

    async fn create_and_send_message(&self, item: &rss::Item) -> Result<()> {
        let title = item.title().unwrap_or("Untitled Course").to_string();

        if let Some(link) = item.link() {
            match extract_udemy_url(link).await {
                Ok(udemy_url) => {
                    let caption = title.clone();
                    let keyboard = InlineKeyboardMarkup::new([vec![InlineKeyboardButton::url(
                        "Get Course".to_string(),
                        udemy_url.clone(),
                    )]]);

                    self.bot
                        .send_photo(self.channel_id, InputFile::url(udemy_url.clone()))
                        .caption(caption)
                        .reply_markup(keyboard)
                        .await?;
                }
                Err(e) => {
                    eprintln!("Failed to extract Udemy URL: {e}");
                }
            }
        }

        Ok(())
    }

    async fn fetch_and_send_new_entries(&self) -> Result<()> {
        let response = reqwest::get(&self.feed_url).await?.bytes().await?;
        let channel = Channel::read_from(&response[..]).context("Failed to parse RSS feed")?;

        let mut seen_entries = self.seen_entries.lock().await;

        for item in channel.items() {
            let entry_id = item
                .guid()
                .map(|g| g.value())
                .or_else(|| item.link())
                .unwrap_or("unknown")
                .to_string();

            if !seen_entries.contains(&entry_id) {
                match self.create_and_send_message(item).await {
                    Ok(_) => {
                        seen_entries.put(entry_id, ());
                    }
                    Err(e) => {
                        eprintln!("Failed to process entry: {e}");
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn start_tracking(&self, interval_minutes: u64) -> Result<()> {
        println!("RSS Bot started. Tracking feed: {}", self.feed_url);

        loop {
            match self.fetch_and_send_new_entries().await {
                Ok(_) => println!("Checked feed successfully."),
                Err(e) => eprintln!("Error: {e}"),
            }

            sleep(Duration::from_secs(interval_minutes * 60)).await;
        }
    }
}