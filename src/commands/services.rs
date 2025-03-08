use serenity::model::channel::Message;
use serenity::prelude::*;
use async_trait::async_trait;
use crate::commands::Command;
use std::process::Command as ProcessCommand;

pub struct ServicesCommand;

impl ServicesCommand {
    pub fn new() -> Self {
        Self {}
    }

    fn get_services(&self) -> Result<String, String> {
        let output = ProcessCommand::new("sh")
            .arg("-c")
            .arg("systemctl list-units --type=service")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(format!("Error: {}", String::from_utf8_lossy(&output.stderr)))
                }
            },
            Err(e) => Err(format!("Failed to fetch services: {}", e))
        }
    }
}

#[async_trait]
impl Command for ServicesCommand {
    fn name(&self) -> &str {
            "services"
    }

    fn description(&self) -> &str {
        "List the running services on the system"
    }

    async fn execute(&self, ctx: &Context, msg: &Message, args: &str) -> Result<(), String> {
        let filter = if !args.is_empty() { Some(args)} else { None };

        let services = self.get_services()?;

        let filtered_services = match filter {
            Some(term) => {
                let lines: Vec<&str> = services.lines()
                    .filter(|line| line.to_lowercase().contains(&term.to_lowercase()))
                    .collect();
                lines.join("\n")
            },
            None => services
        };

        if filtered_services.is_empty() {
            if let Err(why) = msg.channel_id.say(&ctx.http, "No services found.").await {
                return Err(format!("Error sending message: {:?}", why));
            }
            return Ok(());
        }

        let message_chunks = crate::utils::message::split_message(&format!("```\n{}\n```", filtered_services), 2000);

        for chunk in message_chunks {
            if let Err(why) = msg.channel_id.say(&ctx.http, chunk).await {
                return Err(format!("Error sending message: {:?}", why));
            }
        }
        
        Ok(())
    }
}
