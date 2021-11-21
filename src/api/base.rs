use reqwest::blocking::Client as HttpClient;
use serde::de;
use serde;

use crate::api;

pub fn get<T: de::DeserializeOwned>(
    config: &api::config::BotConfig,
    endpoint: String,
) -> T {
    let url = format!("https://discordapp.com/api/v9/{}", endpoint);
    return HttpClient::new()
        .get(url)
        .header("Authorization", format!("Bot {}", config.token))
        .send().unwrap()
        .json().unwrap();
}

pub fn post<T: de::DeserializeOwned, H: serde::Serialize>(
    config: &api::config::BotConfig,
    endpoint: String,
    body: H
) -> T {
    let url = format!("https://discordapp.com/api/v9/{}", endpoint);
    HttpClient::new()
        .post(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bot {}", config.token))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send().unwrap()
        .json().unwrap()
}
