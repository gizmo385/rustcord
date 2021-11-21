use crate::api;

use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

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
    pub author: api::user::User,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mention_roles: Vec<String>,
    pub application: Option<api::misc::Application>,
    pub application_id: Option<String>,
    pub flags: Option<i64>,
    #[serde(rename(deserialize = "type"))]
    pub message_type: MessageType,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SendMessageContent {
    TextMessage(String),
    StickersMessage(Vec<String>),
}

#[derive(Debug, Serialize)]
pub struct SendMessageBuilder {
    tts: bool,
    content: SendMessageContent,
}

impl SendMessageBuilder {
    pub fn new(content: SendMessageContent) -> Self {
        Self {
            content,
            tts: false,
        }
    }

    pub fn tts(&mut self, tts: bool) -> &mut Self {
        self.tts = tts;
        return self;
    }

    pub fn send_to(&self, config: &api::config::BotConfig, channel: &Channel) -> Message {
        api::base::post(config, format!("channels/{}/messages", channel.id), self)
    }
}

impl Channel {
    pub fn get(config: &api::config::BotConfig, channel_id: String) -> Self {
        api::base::get(config, format!("channels/{}", channel_id))
    }

    pub fn create_message(
        &self,
        config: &api::config::BotConfig,
        message: &SendMessageBuilder,
    ) -> Message {
        api::base::post(config, format!("channels/{}/messages", self.id), message)
    }
}
