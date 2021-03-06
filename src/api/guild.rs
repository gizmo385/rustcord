use crate::api;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnavailableGuild {
    pub id: api::misc::Snowflake,
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub id: api::misc::Snowflake,
    pub name: String,
    pub color: i64,
    pub hoist: bool,
    pub position: i64,
    pub permissions: i64,
    pub permissions_new: String,
    pub managed: bool,
    pub mentionable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Guild {
    pub id: api::misc::Snowflake,
    pub name: String,
    pub permissions: Option<String>,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub roles: Vec<Role>,
    pub afk_timeout: i64,
    pub verification_level: i64,
    pub mfa_level: i64,
    pub channels: Option<Vec<api::channel::Channel>>,
}

#[derive(Debug, Deserialize)]
pub struct GuildMember {
    pub user: Option<api::user::User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename(deserialize = "roles"))]
    pub role_ids: Vec<api::misc::Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GuildMemberUpdate {
    pub guild_id: api::misc::Snowflake,
    pub roles: Vec<api::misc::Snowflake>,
    pub user: api::user::User,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub joined_at: Option<String>,
    pub premium_since: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
}

impl Guild {
    pub fn get(config: &api::config::BotConfig, guild_id: api::misc::Snowflake) -> Self {
        api::base::get(config, format!("guilds/{}", guild_id))
    }

    pub fn members(&self, config: &api::config::BotConfig) -> Vec<GuildMember> {
        api::base::get(config, format!("guilds/{}/members", self.id))
    }

    pub fn member(
        &self,
        config: &api::config::BotConfig,
        user_id: api::misc::Snowflake,
    ) -> Option<GuildMember> {
        api::base::get(config, format!("guilds/{}/members/{}", self.id, user_id))
    }

    pub fn roles(&self, config: &api::config::BotConfig) -> Vec<Role> {
        api::base::get(config, format!("guild/{}/roles", self.id))
    }
}
