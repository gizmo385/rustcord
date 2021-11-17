use crate::api;
use crate::types;

pub fn get_channel(config: &api::config::BotConfig, channel_id: String) -> types::Channel {
    let endpoint = format!("channels/{}", channel_id);
    return api::get::<types::Channel>(config, api::Version::V9, endpoint)
}
