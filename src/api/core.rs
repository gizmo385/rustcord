use reqwest::blocking::Client as HttpClient;
use serde::de;
use std::fmt;

use crate::api::config;

pub enum Version {
    V6,
    V8,
    V9,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Version::V9 => "v9",
                Version::V8 => "v8",
                Version::V6 => "v6",
            }
        )
    }
}

pub enum Endpoint {
    BotGateway,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Endpoint::BotGateway => "gateway/bot",
            }
        )
    }
}

pub fn send_discord_request<T: de::DeserializeOwned>(
    config: &config::BotConfig,
    version: Version,
    endpoint: Endpoint,
) -> T {
    let url = format!("https://discordapp.com/api/{}/{}", version, endpoint);
    return HttpClient::new()
        .get(url)
        .header("Authorization", format!("Bot {}", config.token))
        .send().unwrap()
        .json().unwrap();
}
