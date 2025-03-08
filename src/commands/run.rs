use serenity::model::channel::Message;
use serenity::prelude::*;
use std::process::Command as ProcessCommand;
use async_trait::async_trait;
use crate::commands::Command;
use crate::utils::message::{split_message, detect_language};

pub struct RunCommand;

impl RunCommand {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for RunCommand {
    fn name(&self) -> &str {
        "run"
    }

    fn description(&self) -> &str {
        "Make jarvis run a shell command."
    }

    async fn execute(&self, ctx: &Context, msg: &Message, args: &str) -> Result<(), String> {
        if args.is_empty() {
            return Err("Provide a command to run".to_string());
        }

        let output = ProcessCommand::new("sh")
            .arg("-c")
            .arg(args)
            .output();
    
    match output {
            Ok(output) => {
                let response = if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).to_string()
                } else {
                    format!("Error: {}", String::from_utf8_lossy(&output.stderr))
                };

                if response.is_empty() {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Command executed successfully with no output.").await {
                        return Err(format!("Error sending message: {:?}", why));
                    }
                    return Ok(());
                }

                let formatted_response = format!("```{}\n{}```", detect_language(args), response);

                for message in split_message(&formatted_response, 2000) {
                    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                        return Err(format!("Error sending message: {:?}", why));
                    }
                }
                Ok(())
            }
            Err(e) => {
                Err(format!("Failed to execute command: {}", e))
            }
        }
    }
}
