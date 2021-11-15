use std::{fs, io};

pub mod api;
pub mod gateway;
pub mod types;

fn main() -> io::Result<()> {
    let config_file_path = fs::canonicalize("./config/config.json")?;
    let bot_config = api::config::load_config(config_file_path)?;
    let gateway_config = api::get_bot_gateway(&bot_config);

    gateway::connect_to_gateway(bot_config, gateway_config);


    Ok(())
}
