use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::info;

pub struct ReadyHandler;

impl ReadyHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self, _ctx: &Context, ready: &Ready) {
        info!("{} is connected", ready.user.name);
    }
}
