use crate::api;
use crate::types;

use std::net::TcpStream;
//use std::{thread, time};
use serde::de;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::atomic;

use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

type GatewayWebSocket = WebSocket<MaybeTlsStream<TcpStream>>;

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
        Self: std::fmt::Debug,
    {
        let message = RawSendableGatewayMessage {
            op: self.opcode(),
            d: self,
        };
        println!("Sending message: {:#?}", message);
        let formatted_payload = Message::Text(serde_json::to_string(&message).unwrap());
        websocket.write_message(formatted_payload).unwrap();

        ();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Receiving generic messages from the gateway websocket
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
struct RawReceivableGatewayMessage<T> {
    op: i32,
    d: T,
}

pub trait ReceivableMessage<T: std::fmt::Debug + de::DeserializeOwned> {
    fn from_websocket(ws: &mut GatewayWebSocket) -> T {
        let raw_message = ws.read_message().unwrap().to_string();
        let parsed_message: RawReceivableGatewayMessage<T> =
            serde_json::from_str(&raw_message).unwrap();
        println!("Received message from gateway: {:#?}", parsed_message);
        return parsed_message.d;
    }
}

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
    fn from_config(config: api::config::BotConfig) -> Self {
        Identify {
            token: config.token,
            intents: config.intents,
            properties: ConnectionProperties {
                os: "test".to_string(),
                browser: "test".to_string(),
                device: "test".to_string(),
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
/// Receiving the HELLO message from the gateway
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, Debug)]
struct Hello {
    heartbeat_interval: u32,
}

impl ReceivableMessage<Hello> for Hello {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Receiving the READY message from the gateway
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, Debug)]
struct Ready {
    #[serde(rename(deserialize = "v"))]
    version: i32,
    user: types::User,
    session_id: String,
    application: types::Application,
}

impl ReceivableMessage<Ready> for Ready {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Heartbeats to and from the gateway
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize)]
struct Heartbeat {
    d: i64,
}
impl SendableGatewayMessage for Heartbeat {
    fn opcode(&self) -> u32 {
        1
    }
}

impl Heartbeat {
    fn from_atom(sequence_num: &atomic::AtomicI64) -> Self {
        Heartbeat {
            d: sequence_num.load(atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Deserialize)]
struct HeartbeatAck {}
impl ReceivableMessage<HeartbeatAck> for HeartbeatAck {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Establishing a connection to the gateway
////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn connect_to_gateway(
    bot_config: api::config::BotConfig,
    gateway_config: api::GatewayBotResponse,
) -> () {
    let (mut ws, _response) =
        connect(Url::parse(&gateway_config.url).unwrap()).expect("Can't connect");

    // After initially connecting to the gateway, we should expect to receive a HELLO message from
    // the gateway that tells us what our heartbeat interval will be.
    let hello_message = Hello::from_websocket(&mut ws);
    println!("Hello: #{:?}", &hello_message);

    // Once we receive that, we'll send an IDENTITY message to the gateway, providing information
    // about ourselves.
    Identify::from_config(bot_config).send(&mut ws);

    // After sending the identify, we should expect to receive the READY message, which will give
    // us additional information about our connection session.
    let ready_message = Ready::from_websocket(&mut ws);
    println!("READY: #{:?}", &ready_message);

    // And now we need to start a background thread that will send heartbeat messages. To do this,
    // we need to create a bit of shared memory that our gateway can use to track the current
    // sequence number for messages coming from the gateway.
    println!(
        "Sending a heartbeat every {:#?} milliseconds",
        &hello_message.heartbeat_interval
    );

    let seq_num = atomic::AtomicI64::new(0);
    Heartbeat::from_atom(&seq_num).send(&mut ws);

    HeartbeatAck::from_websocket(&mut ws);
}
