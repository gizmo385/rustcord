use std::collections::HashMap;

use crate::api;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct UnavailableGuild {
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: i64,
    pub hoist: bool,
    pub position: i64,
    pub permissions: i64,
    pub permissions_new: String,
    pub managed: bool,
    pub mentionable: bool
}

#[derive(Debug, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub permissions: Option<String>,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub roles: Vec<Role>,
    pub afk_timeout: i64,
    pub verification_level: i64,
    pub mfa_level: i64,
}

#[derive(Debug, Deserialize)]
pub struct GuildMember {
    pub user: Option<api::user::User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename(deserialize="roles"))]
    pub role_ids: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResolvedCommandData {
    pub users: Option<HashMap<String, api::user::User>>,
    pub members: Option<HashMap<String, GuildMember>>,
    pub roles: Option<HashMap<String, Role>>,
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
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub member: Option<GuildMember>,
    pub user: Option<api::user::User>,
    pub token: String,
    pub version: i64,
    pub message: Option<api::channel::Message>,
    pub data: InteractionData,
    #[serde(rename(deserialize = "type"))]
    pub interaction_type: InteractionType,
}
