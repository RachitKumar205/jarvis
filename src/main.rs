use serenity::prelude::*;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod commands;
mod config;
mod handlers;
mod utils;

use config::BotConfig;
use handlers::Handler;

#[tokio::main]
async fn main() {
    let config = BotConfig::load();

    let log_level = match config.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up logging");

    info!("Booting up Jarvis with prefix: {}", config.prefix);

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler::new(&config))
        .await
        .expect("Error creating client");
    
    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }
}

