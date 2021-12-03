use crate::api;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

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
    pub id: api::misc::Snowflake,
    pub author: api::user::User,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub content: String,
    pub timestamp: i64,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mention_roles: Vec<String>,
    pub application: Option<api::misc::Application>,
    pub application_id: Option<api::misc::Snowflake>,
    pub flags: Option<i64>,
    #[serde(rename(deserialize = "type"))]
    pub message_type: MessageType,
}

#[derive(Clone, Debug)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Debug)]
pub struct MessageEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<i64>,
    pub color: Option<i64>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Vec<EmbedField>,
}

pub struct EmbedBuilder(MessageEmbed);

impl EmbedBuilder {
    pub fn new() -> Self {
        Self(MessageEmbed {
            title: None,
            description: None,
            url: None,
            timestamp: None,
            color: None,
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: Vec::new(),
        })
    }

    pub fn title(&mut self, t: String) -> &mut Self {
        self.0.title = Some(t);
        self
    }

    pub fn description(&mut self, desc: String) -> &mut Self {
        self.0.description = Some(desc);
        self
    }

    pub fn url(&mut self, u: String) -> &mut Self {
        self.0.title = Some(u);
        self
    }

    pub fn timestamp(&mut self, t: i64) -> &mut Self {
        self.0.timestamp = Some(t);
        self
    }

    pub fn color(&mut self, c: i64) -> &mut Self {
        self.0.color = Some(c);
        self
    }
}
