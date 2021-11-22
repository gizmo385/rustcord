use crate::api;
use serde::{Deserialize, Serialize};
use serde::de;
use std::{fmt, hash::Hash, ops::{Deref, DerefMut}};

/// [Snowflakes](https://discord.com/developers/docs/reference#snowflakes) are a data type that
/// Discord leverages for unique identifiers. Alongside being a unique identifier, they also have
/// several pieces of information encoded within them:
/// - The timestamp at which the referenced item was created.
/// - The internal ID of the worker that generated it.
/// - The internal ID of the process that generated it.
/// - And an internal incrementing ID.
#[derive(Debug, Serialize, Hash, PartialEq, Eq)]
pub struct Snowflake(u64);

impl Deref for Snowflake {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Snowflake {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Functions for creating snowflakes as well as retrieving the information contained within them.
impl Snowflake {
    /// Many APIs within Discord return their snowflakes from Strings, so this helper function is
    /// can be used to coerce those strings into Snowflakes.
    pub fn from_string(v: String) -> Self {
        Self(v.parse().expect("Invalid snowflake!"))
    }

    /// Returns the Unix timestamp encoded within the snowflake
    pub fn timestamp(&self) -> u64 {
        (self.0 >> 22) + 1420070400000
    }

    /// Returns the internal worker ID encoded within the snowflake
    pub fn internal_worker_id(&self) -> i8 {
        ((self.0 & 0x3E0000) >> 17) as i8
    }

    /// Returns the internal process ID encoded within the snowflake
    pub fn internal_process_id(&self) -> i8 {
        ((self.0 & 0x1F000) >> 12) as i8
    }

    /// Returns the internal increment ID encoded within the snowflake
    pub fn increment_id(&self) -> i16 {
        (self.0 & 0xFFF) as i16
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct IdVisitor;

        impl<'de> de::Visitor<'de> for IdVisitor {
            type Value = Snowflake;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("Snowflake as a number or string")
            }

            fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Snowflake(id))
            }

            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                id.parse().map(Snowflake).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

/// An [Application](https://discord.com/developers/docs/resources/application) within Discord and
/// describes meta-information about the bot.
#[derive(Debug, Deserialize)]
pub struct Application {
    /// The unique identifier for this application
    pub id: Snowflake,
}

#[derive(Debug, Deserialize)]
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u32,
    pub max_concurrency: u32,
}

/// [Information](https://discord.com/developers/docs/topics/gateway#get-gateway-bot) about how
/// the bot should connect to the Discord Gateway.
#[derive(Debug, Deserialize)]
pub struct BotGateway {
    /// The URL of the gateay that the bot should connect to
    pub url: String,
    /// The recommended number of shards that should be used when connecting to the gateway
    pub shards: u32,
    /// Metadata about the start of the session.
    pub session_start_limit: SessionStartLimit,
}

impl BotGateway {
    pub fn get(config: &api::config::BotConfig) -> Self {
        api::base::get(config, String::from("gateway/bot"))
    }
}

