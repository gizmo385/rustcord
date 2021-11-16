use std::{fs, io};

pub mod api;
pub mod gateway;
pub mod types;

fn main() -> io::Result<()> {
    let config_file_path = fs::canonicalize("./config/config.json")?;
    let bot_config = api::config::load_config(config_file_path)?;
    let gateway_config = api::get_bot_gateway(&bot_config);

    let mut connection = gateway::connect_to_gateway(bot_config, gateway_config);
    println!("Established connection to the gateway: {:#?}", connection);

    loop {
        if let Some(next_message) = connection.read_event() {
            match next_message.data {
                gateway::GatewayMessageData::HeartbeatAck(_) => {
                    println!("Received heartbeat HeartbeatAck");
                }
                gateway::GatewayMessageData::GuildCreate(guild) => {
                    println!("Guild info: {:#?}", guild);
                }
            }
        }
    }
}
