use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub application_id: String,
    pub intents: u32,
}

pub fn load_config(filename: PathBuf) -> io::Result<BotConfig> {
    let config_content = fs::read_to_string(filename)?;
    let parsed_config = serde_json::from_str(&config_content)?;
    Ok(parsed_config)
}
