use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub verified: Option<bool>,
    pub locale: Option<String>,
    pub public_flags: Option<i64>,
    pub premium_type: Option<i64>,
    pub flags: Option<i64>,
}

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

#[derive(Debug, Deserialize_repr)]
#[repr(i64)]
pub enum ChannelType {
    GuildText = 0,
    DirectMessage = 1,
    GuildVoice = 2,
    GroupDirectMessage = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub channel_type: ChannelType,
    pub guild_id: Option<String>,
    pub position: Option<i64>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
}

#[derive(Debug, Deserialize_repr)]
#[repr(i64)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    pub author: User,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mention_roles: Vec<String>,
    pub application: Option<Application>,
    pub application_id: Option<String>,
    pub flags: Option<i64>,
    #[serde(rename(deserialize = "type"))]
    pub message_type: MessageType,
}
