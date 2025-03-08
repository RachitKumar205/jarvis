use std::env;
use dotenv::dotenv;

pub struct BotConfig {
    pub token: String,
    pub prefix: String,
    pub owner_id: Option<u64>,
    pub log_level: String,
}

impl BotConfig {
    pub fn load() -> Self{
        dotenv().ok();

        Self {
            token: env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN not set"),
            prefix: env::var("BOT_PREFIX").unwrap_or_else(|_| ">>jarvis, ".to_string()),
            owner_id: env::var("OWNER_ID").ok().and_then(|id| id.parse().ok()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        }
    }
}
