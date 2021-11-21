use crate::api;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub verified: Option<bool>,
    pub locale: Option<String>,
    pub public_flags: Option<i64>,
    pub premium_type: Option<i64>,
    pub flags: Option<i64>,
}

impl User {
    pub fn current(config: &api::config::BotConfig) -> Self {
        api::base::get(config, String::from("users/@me"))
    }

    pub fn get(config: &api::config::BotConfig, user_id: String) -> Self {
        api::base::get(config, format!("users/{}", user_id))
    }

    pub fn dm_channel(&self, config: &api::config::BotConfig) -> api::channel::Channel {
        let mut payload = HashMap::new();
        payload.insert(String::from("recipient_id"), self.id.clone());
        api::base::post(config, String::from("users/@me/channels"), payload)
    }
}
