use std::collections::HashMap;

use crate::api;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub struct ResolvedCommandData {
    pub users: Option<HashMap<api::misc::Snowflake, api::user::User>>,
    pub members: Option<HashMap<api::misc::Snowflake, api::guild::GuildMember>>,
    pub roles: Option<HashMap<api::misc::Snowflake, api::guild::Role>>,
    pub channels: Option<HashMap<api::misc::Snowflake, api::channel::Channel>>,
    pub messages: Option<HashMap<api::misc::Snowflake, api::channel::Message>>,
}

#[derive(Debug, Deserialize)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3
}

#[derive(Debug, Deserialize)]
pub struct ApplicationCommandData {
    pub id: api::misc::Snowflake,
    pub name: String,
    pub resolved: Option<ResolvedCommandData>,
    #[serde(rename(deserialize = "type"))]
    pub command_type: ApplicationCommandType,
}

#[derive(Debug, Deserialize_repr)]
#[repr(i64)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
}

#[derive(Debug, Deserialize)]
pub struct InteractionOption {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename(deserialize="type"))]
    pub option_type: ApplicationCommandOptionType,
    pub focused: Option<bool>,
    pub options: Option<Vec<InteractionOption>>,
}

#[derive(Debug, Deserialize)]
pub struct InteractionData {
    pub id: api::misc::Snowflake,
    pub name: String,
    pub options: Option<Vec<InteractionOption>>,
}

#[derive(Debug, Deserialize_repr)]
#[repr(i64)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4
}

#[derive(Debug, Deserialize)]
pub struct Interaction {
    pub id: api::misc::Snowflake,
    pub application_id: String,
    pub guild_id: Option<api::misc::Snowflake>,
    pub channel_id: Option<api::misc::Snowflake>,
    pub member: Option<api::guild::GuildMember>,
    pub user: Option<api::user::User>,
    pub token: String,
    pub version: i64,
    pub message: Option<api::channel::Message>,
    pub data: InteractionData,
    #[serde(rename(deserialize = "type"))]
    pub interaction_type: InteractionType,
}
