use std::sync::atomic;
use std::{fs, io};

pub mod api;
pub mod gateway;
pub mod types;

fn listen_for_messages(connection: &mut gateway::GatewayConnection) -> () {
    loop {
        if let Some(next_message) = connection.read_event() {
            // Update the sequence number on each message
            if let Some(seq_num) = next_message.sequence_number {
                println!("Updating seq num to {:#?}", seq_num);
                connection
                    .sequence_number
                    .fetch_max(seq_num, atomic::Ordering::Relaxed);
            }

            // Handle the message data
            match next_message.data {
                gateway::GatewayMessageData::HeartbeatAck(_) => {
                    println!("Received heartbeat HeartbeatAck");
                }
                gateway::GatewayMessageData::GuildCreate(guild) => {
                    println!("Guild info: {:#?}", guild);
                }
                gateway::GatewayMessageData::MessageCreate(message) => {
                    println!(
                        "Message created by {:#?}: {:#?}",
                        message.author.username, message
                    );
                }
                gateway::GatewayMessageData::InteractionCreate(interaction) => {
                    println!("Received interaction: {:#?}", interaction)
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let config_file_path = fs::canonicalize("./config/config.json")?;
    let bot_config = api::config::load_config(config_file_path)?;
    let gateway_config = api::misc::BotGatewayInformation::get(&bot_config);
    let mut connection = gateway::connect_to_gateway(&bot_config, gateway_config);
    listen_for_messages(&mut connection);

    return Ok(());
}
