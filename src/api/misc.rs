use crate::api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayBotResponse {
    pub url: String,
    pub shards: u32,
}

pub fn get_bot_gateway(config: &api::config::BotConfig) -> GatewayBotResponse {
    return api::get::<GatewayBotResponse>(
        config,
        api::Version::V9,
        String::from("gateway/bot"),
    );
}
