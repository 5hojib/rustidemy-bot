use anyhow::{Context, Result};
use std::env;
use teloxide::types::ChatId;

pub struct Config {
    pub bot_token: String,
    pub channel_id: ChatId,
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

        Ok(Config {
            bot_token,
            channel_id,
        })
    }
}
