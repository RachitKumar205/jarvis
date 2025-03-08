use serenity::model::channel::Message;
use serenity::prelude::*;
use async_trait::async_trait;
use crate::commands::{Command, CommandRegistry};
use serenity::utils::Colour;
use std::sync::Arc;

pub struct HelpCommand {
    registry: Arc<CommandRegistry>,
}

impl HelpCommand {
    pub fn new(registry: Arc<CommandRegistry>) -> Self {
        Self {
            registry
        }
    }
}

#[async_trait]
impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Get the list of available commands."
    }

    async fn execute(&self, ctx: &Context, msg: &Message, args: &str) -> Result<(), String> {
        let prefix = self.registry.get_prefix();

        if !args.is_empty() {
            if let Some(command) = self.registry.get_command(args) {
                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title(format!("Help: {} {}", prefix, command.name()))
                         .description(command.description())
                         .colour(Colour::BLUE)
                    })
                }).await.map_err(|e| format!("Error sending help message: {:?}", e))?;

                return Ok(());
            } else {
                return Err(format!("Command `{} {}` not found.", prefix, args));
            }
        }

        let mut commands_list = String::new();

        for (name, command) in self.registry.get_commands() {
            commands_list.push_str(&format!("**{} {}** - {}\n", prefix, name, command.description()));
        }

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Available Commands")
                 .description(&commands_list)
                 .colour(Colour::BLUE)
                 .footer(|f| {
                    f.text(format!("Type {} help <command> to know more about the command", prefix))
                 })
            })
        }).await.map_err(|e| format!("Error sending help message: {:?}", e))?;

        Ok(())
    }
}
