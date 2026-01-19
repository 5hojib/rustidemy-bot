use anyhow::{Context, Result};
use std::env;
use teloxide::types::ChatId;

pub struct Config {
    pub bot_token: String,
    pub channel_id: ChatId,
    pub feed_url: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let bot_token = env::var("BOT_TOKEN").context("BOT_TOKEN not set")?;
        let channel_id_str = env::var("CHANNEL_ID").context("CHANNEL_ID not set")?;
        let channel_id = ChatId(
            channel_id_str
                .parse()
                .context("Failed to parse CHANNEL_ID")?,
        );
        let feed_url =
            env::var("FEED_URL").unwrap_or_else(|_| "https://www.discudemy.com/feed".to_string());

        Ok(Config {
            bot_token,
            channel_id,
            feed_url,
        })
    }
}
