mod message;
mod ready;

use message::MessageHandler;
use ready::ReadyHandler;
use async_trait::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use crate::config::BotConfig;

pub struct Handler {
    message_handler: MessageHandler,
    ready_handler: ReadyHandler,
}

impl Handler {
    pub fn new(config: &BotConfig) -> Self {
        Self {
            message_handler: MessageHandler::new_with_prefix(&config.prefix),
            ready_handler: ReadyHandler::new(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.message_handler.handle(&ctx, &msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        self.ready_handler.handle(&ctx, &ready).await;
    }
}
