use serenity::model::channel::Message;
use serenity::prelude::*;
use async_trait::async_trait;
use crate::commands::Command;
use std::time::{Instant, Duration};

pub struct PingCommand;

impl PingCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &str {
        "ping"
    }

    fn description(&self) -> &str {
        "Check the bot's latency"
    }

    async fn execute(&self, ctx: &Context, msg: &Message, _args: &str) -> Result<(), String> {
        let start = Instant::now();

        let mut response = msg.channel_id.say(&ctx.http, "Pinging...").await
            .map_err(|e| format!("Error sending ping message: {:?}", e))?;

        let latency = start.elapsed();

        let content = format!("Jarvis is online. Latency: {}ms", latency.as_millis());

        response.edit(&ctx.http, |m| m.content(content)).await
            .map_err(|e| format!("Error editing ping message: {:?}", e))?;

        Ok(())
    }
}
