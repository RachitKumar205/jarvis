use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::commands::{CommandRegistry, HelpCommand, Command};
use std::sync::Arc;

pub struct MessageHandler {
    registry: Arc<CommandRegistry>,
}

impl MessageHandler {
    pub fn new() -> Self {
        Self::new_with_prefix(">>jarvis, ")
    }

    pub fn new_with_prefix(prefix: &str) -> Self {
        let registry = Arc::new(CommandRegistry::new(prefix));

        let registry_clone = registry.clone();

        registry.register(Arc::new(HelpCommand::new(registry_clone)));

        Self {
            registry
        }
    }

    pub async fn handle(&self, ctx: &Context, msg: &Message) {
        if msg.author.bot {
            return;
        }

        let _ = self.registry.handle_message(ctx, msg).await;
    }
}
