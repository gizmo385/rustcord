pub mod config;
pub mod misc;
pub mod channel;

use reqwest::blocking::Client as HttpClient;
use serde::de;
use serde;

pub enum Version {
    V6,
    V8,
    V9,
}

pub fn get<T: de::DeserializeOwned>(
    config: &config::BotConfig,
    version: Version,
    endpoint: String,
) -> T {
    let version_string = String::from(match version {
        Version::V6 => "v6",
        Version::V8 => "v8",
        Version::V9 => "v9",
    });
    let url = format!("https://discordapp.com/api/{}/{}", version_string, endpoint);
    return HttpClient::new()
        .get(url)
        .header("Authorization", format!("Bot {}", config.token))
        .send().unwrap()
        .json().unwrap();
}

pub fn post<T: de::DeserializeOwned, H: serde::Serialize>(
    config: &config::BotConfig,
    version: Version,
    endpoint: String,
    body: H
) -> T {
    let version_string = String::from(match version {
        Version::V6 => "v6",
        Version::V8 => "v8",
        Version::V9 => "v9",
    });
    let url = format!("https://discordapp.com/api/{}/{}", version_string, endpoint);
    return HttpClient::new()
        .post(url)
        .header("Authorization", format!("Bot {}", config.token))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send().unwrap()
        .json().unwrap();
}
