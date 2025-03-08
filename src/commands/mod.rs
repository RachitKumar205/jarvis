mod run;
mod ping;
mod services;
mod help;

use serenity::model::channel::Message;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, ctx: &Context, msg: &Message, args: &str) -> Result<(), String>;
}

pub use run::RunCommand;
pub use ping::PingCommand;
pub use services::ServicesCommand;
pub use help::HelpCommand; 

pub struct CommandRegistry {
    prefix: String,
    commands: Mutex<HashMap<String, Arc<dyn Command>>>,
}

impl CommandRegistry {
    pub fn new(prefix: &str) -> Self {
        let mut registry = Self {
            prefix: prefix.to_string(),
            commands: Mutex::new(HashMap::new()),
        };

        registry.register(Arc::new(RunCommand::new()));
        registry.register(Arc::new(PingCommand::new()));
        registry.register(Arc::new(ServicesCommand::new()));

        registry
    }

    pub fn register(&self, command: Arc<dyn Command>){
        let mut commands = self.commands.lock().unwrap();
        commands.insert(command.name().to_string(), command);
    }

    pub fn get_command(&self, name: &str) -> Option<Arc<dyn Command>> {
        let commands = self.commands.lock().unwrap();
        commands.get(name).cloned()
    }

    pub fn get_commands(&self) -> HashMap<String, Arc<dyn Command>> {
        let commands = self.commands.lock().unwrap();
        commands.clone()
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    pub async fn handle_message(&self, ctx: &Context, msg: &Message) -> bool {
        let full_prefix = format!("{}", self.prefix);

        if !msg.content.starts_with(&full_prefix) {
            return false;
        }

        let without_prefix = msg.content.trim_start_matches(&full_prefix).trim();

        let mut parts = without_prefix.splitn(2, ' ');
        let command_name = parts.next().unwrap_or("").trim();
        let args = parts.next().unwrap_or("").trim();

        let command = self.get_command(command_name);

        if let Some(command) = command {
            if let Err(error) = command.execute(ctx, msg, args).await {
                if let Err(why) = msg.channel_id.say(&ctx.http, &error).await {
                    println!("Error sending error message: {:?}", why);
                }
            }
            return true;
        }
    
        if !command_name.is_empty() {
            if let Err(why) = msg.channel_id.say(
                &ctx.http,
                format!("Unknown command: `{}`. Use `{}help` to see available commands.", command_name, self.prefix)
            ).await {
                println!("Error sending unknown command message: {:?}", why);
            }
            return true;
        }

        false

    }
}
