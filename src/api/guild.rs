use crate::api;
use serde::Deserialize;

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


impl Guild {
    pub fn get(config: &api::config::BotConfig, guild_id: String) -> Self {
        api::base::get(config, format!("guilds/{}", guild_id))
    }

    pub fn members(&self, config: &api::config::BotConfig) -> Vec<GuildMember> {
        api::base::get(config, format!("guilds/{}/members", self.id))
    }

    pub fn member(&self, config: &api::config::BotConfig, user_id: String) -> Option<GuildMember> {
        api::base::get(config, format!("guilds/{}/members/{}", self.id, user_id))
    }

    pub fn roles(&self, config: &api::config::BotConfig) -> Vec<Role> {
        api::base::get(config, format!("guild/{}/roles", self.id))
    }
}
