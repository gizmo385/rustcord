use crate::api;
use crate::types;

use serde::de;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::net::TcpStream;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

// This is a type alias for the type of web socket that we'll be opening
pub type GatewayWebSocket = WebSocket<MaybeTlsStream<TcpStream>>;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Sending generic messages to the gateway websocket
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Debug)]
struct RawSendableGatewayMessage<T> {
    op: u32,
    d: T,
}

pub trait SendableGatewayMessage {
    fn opcode(&self) -> u32;

    fn send(&self, websocket: &mut GatewayWebSocket) -> ()
    where
        Self: Serialize,
    {
        let message = RawSendableGatewayMessage {
            op: self.opcode(),
            d: self,
        };
        let formatted_payload = Message::Text(serde_json::to_string(&message).unwrap());
        websocket.write_message(formatted_payload).unwrap();

        ();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Messages that we can force receive from the gateway. These messages are received in
/// a particular order while we're initializing our connection with the gateway.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
struct PrivateGatewayEvent<T> {
    pub op: i64,
    pub s: Option<i64>,
    pub t: Option<String>,
    pub d: T,
}

trait ExpectableWebsocketMessage<T: std::fmt::Debug + de::DeserializeOwned> {
    fn expect_from_websocket(ws: &mut GatewayWebSocket) -> T {
        let raw_message = ws.read_message().unwrap().to_string();
        let parsed_message: PrivateGatewayEvent<T> = serde_json::from_str(&raw_message).unwrap();
        println!("Received message from gateway: {:#?}", parsed_message);
        return parsed_message.d;
    }
}

#[derive(Deserialize, Debug)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

impl ExpectableWebsocketMessage<Hello> for Hello {}

#[derive(Deserialize, Debug)]
pub struct Ready {
    #[serde(rename(deserialize = "v"))]
    pub version: i32,
    pub user: types::User,
    pub session_id: String,
    pub application: types::Application,
    pub guilds: Vec<types::UnavailableGuild>,
}

impl ExpectableWebsocketMessage<Ready> for Ready {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Sending the IDENTIFY message to the gateway
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize)]
struct ConnectionProperties {
    #[serde(rename(serialize = "$os"))]
    os: String,

    #[serde(rename(serialize = "$browser"))]
    browser: String,

    #[serde(rename(serialize = "$device"))]
    device: String,
}

#[derive(Debug, Serialize)]
struct Identify {
    token: String,
    intents: u32,
    properties: ConnectionProperties,
}

impl Identify {
    fn from_config(config: &api::config::BotConfig) -> Self {
        Identify {
            token: String::from(&config.token),
            intents: config.intents,
            properties: ConnectionProperties {
                os: env::consts::OS.to_string(),
                browser: "rustcord".to_string(),
                device: "rustcord".to_string(),
            },
        }
    }
}

impl SendableGatewayMessage for Identify {
    fn opcode(&self) -> u32 {
        2
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Heartbeats to and from the gateway
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize)]
pub struct Heartbeat {
    d: i64,
}
impl SendableGatewayMessage for Heartbeat {
    fn opcode(&self) -> u32 {
        1
    }
}

impl Heartbeat {
    pub fn from_atom(sequence_num: &AtomicI64) -> Self {
        Heartbeat {
            d: sequence_num.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeartbeatAck {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// The different types of messages that should be expected from the user.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GatewayMessageData {
    HeartbeatAck(HeartbeatAck),
    GuildCreate(types::Guild),
    MessageCreate(types::Message),
}

#[derive(Debug, Deserialize)]
pub struct GatewayEvent {
    #[serde(rename(deserialize = "op"))]
    pub opcode: i64,

    #[serde(rename(deserialize = "s"))]
    pub sequence_number: Option<i64>,

    #[serde(rename(deserialize = "t"))]
    pub dispatch_type: Option<String>,

    #[serde(rename(deserialize = "d"))]
    pub data: GatewayMessageData,
}

// And this is what we'll construct after connecting to the websocket
#[derive(Debug)]
pub struct GatewayConnection {
    pub websocket: GatewayWebSocket,
    pub sequence_number: Arc<AtomicI64>,
    pub heartbeat_interval: u64,
}

impl GatewayConnection {
    pub fn read_event(&mut self) -> Option<GatewayEvent> {
        GatewayEvent::from_gateway(self)
    }
}

impl GatewayEvent {
    pub fn from_gateway(conn: &mut GatewayConnection) -> Option<Self> {
        // TODO: This doesn't handle errors when reading from the websocket
        let raw_message = conn.websocket.read_message().unwrap().to_string();
        let parsed_message = serde_json::from_str(&raw_message);
        match parsed_message {
            Ok(r) => return Some(r),
            _ => {
                println!("Error deserializing gateway message: {:#?}", raw_message);
                return None;
            }
        };
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Establishing a connection to the gateway
////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn connect_to_gateway(
    bot_config: &api::config::BotConfig,
    gateway_config: api::misc::GatewayBotResponse,
) -> GatewayConnection {
    // Create the initial connection to the websocket
    let gateway_url = Url::parse(&gateway_config.url).expect("Could not parse gatweay URL");
    let (mut websocket, _response) = connect(gateway_url).expect("Can't connect");

    // Once we've actually established a raw connection connected, we'll expect a series of events
    // from the gateway in succession to correctly establish a valid connection. To do this, we'll
    // expect to receive those events from the websocket. The order of operations, as specificed in
    // the developer documentation is:
    //
    // 1. We should expect a HELLO message from the gateway, which will inform us of the interval
    //    at which our client is expected to send heartbeats.
    // 2. We should then send an IDENTIFY message to the gateway, providing our intent, shard, and
    //    connection properties information.
    // 3. Assuming the IDENTIFY message is valid, we should expect to receive a READY message, at
    //    which point we are considered 'connected' to the gateway.
    let hello = Hello::expect_from_websocket(&mut websocket);
    Identify::from_config(bot_config).send(&mut websocket);
    Ready::expect_from_websocket(&mut websocket);

    // Create the gateway object
    GatewayConnection {
        websocket,
        heartbeat_interval: hello.heartbeat_interval,
        sequence_number: Arc::new(AtomicI64::new(0)),
    }
}
