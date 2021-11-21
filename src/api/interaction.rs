use std::collections::HashMap;

use crate::api;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub struct ResolvedCommandData {
    pub users: Option<HashMap<String, api::user::User>>,
    pub members: Option<HashMap<String, api::guild::GuildMember>>,
    pub roles: Option<HashMap<String, api::guild::Role>>,
    pub channels: Option<HashMap<String, api::channel::Channel>>,
    pub messages: Option<HashMap<String, api::channel::Message>>,
}

#[derive(Debug, Deserialize)]
pub enum ApplicationCommandType {
    ChatInput = 1,
    User = 2,
    Message = 3
}

#[derive(Debug, Deserialize)]
pub struct ApplicationCommandData {
    pub id: String,
    pub name: String,
    pub resolved: Option<ResolvedCommandData>,
    #[serde(rename(deserialize = "type"))]
    pub command_type: ApplicationCommandType,
}

#[derive(Debug, Deserialize)]
pub struct InteractionData {
    pub id: String,
    pub name: String,
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
#[serde(untagged)]
pub enum InteractionInvoker {
    Guild(api::guild::GuildMember),
    DirectMessage(api::user::User),
}

#[derive(Debug, Deserialize)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub invoked_by: InteractionInvoker,
    pub member: Option<api::guild::GuildMember>,
    pub user: Option<api::user::User>,
    pub token: String,
    pub version: i64,
    pub message: Option<api::channel::Message>,
    pub data: InteractionData,
    #[serde(rename(deserialize = "type"))]
    pub interaction_type: InteractionType,
}
