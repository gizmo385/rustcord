use crate::api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Application {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotGatewayInformation {
    pub url: String,
    pub shards: u32,
}

impl BotGatewayInformation {
    pub fn get(config: &api::config::BotConfig) -> Self {
        api::base::get(config, String::from("gateway/bot"))
    }
}
