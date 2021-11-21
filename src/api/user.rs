use crate::api;

use std::collections::HashMap;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

pub enum Flag {
    NoFlag = 0,
    Staff = 1 << 0,
    Partner = 1 << 1,
    HypeSquadEventsCoordinator = 1 << 2,
    BugHunterLevel1 = 1 << 3,
    HypeSquadBraveryMember = 1 << 6,
    HypeSquadBrillianceMember = 1 << 7,
    HypeSquadBalanceMember = 1 << 8,
    PremiumEarlySupporter = 1 << 9,
    TeamPseudoUser = 1 << 10,
    BugHunterLevel2 = 1 << 14,
    VerifiedBot = 1 << 16,
    VerifiedDeveloper = 1 << 17,
    CertifiedModerator = 1 << 18,
    BotHttpInteractions = 1 << 19,
}

#[derive(Debug, Deserialize_repr)]
#[repr(i64)]
pub enum PremiumType {
    NoPremium = 0,
    NitroClassic = 1,
    Nitro = 2
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub verified: Option<bool>,
    pub locale: Option<String>,
    pub premium_type: Option<PremiumType>,
    public_flags: Option<u32>,
    flags: Option<u32>,
}

impl User {
    pub fn current(config: &api::config::BotConfig) -> Self {
        api::base::get(config, String::from("users/@me"))
    }

    pub fn get(config: &api::config::BotConfig, user_id: String) -> Self {
        api::base::get(config, format!("users/{}", user_id))
    }

    pub fn dm_channel(&self, config: &api::config::BotConfig) -> api::channel::Channel {
        let mut payload = HashMap::new();
        payload.insert(String::from("recipient_id"), self.id.clone());
        api::base::post(config, String::from("users/@me/channels"), payload)
    }

    pub fn has_flag(&self, flag: Flag) -> Option<bool> {
        let flag_discrim = flag as u32;
        if let Some(user_flags) = self.flags {
            return Some((flag_discrim & user_flags) == flag_discrim)
        } else { None }
    }

    pub fn has_public_flag(&self, flag: Flag) -> Option<bool> {
        let flag_discrim = flag as u32;
        if let Some(public_user_flags) = self.public_flags {
            return Some((flag_discrim & public_user_flags) == flag_discrim)
        } else { None }
    }
}
