use serde::{Deserialize, Serialize};

pub mod config;
pub mod core;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayBotResponse {
    pub url: String,
    pub shards: u32,
}

pub fn get_bot_gateway(config: &config::BotConfig) -> GatewayBotResponse {
    return core::send_discord_request::<GatewayBotResponse>(
        config,
        core::Version::V9,
        core::Endpoint::BotGateway,
    );
}
