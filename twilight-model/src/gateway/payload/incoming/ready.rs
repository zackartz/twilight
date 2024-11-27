use crate::{
    gateway::{
        presence::{
            Activity, ClientStatus, Presence, PresenceIntermediary, PresenceListDeserializer,
            Status,
        },
        ShardId,
    },
    guild::Guild,
    oauth::PartialApplication,
    user::{CurrentUser, User},
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Ready {
//     pub s: i64,
//     pub op: i64,
//     pub d: D,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ready {
    pub session_type: String,
    pub api_code_version: i64,
    pub presences: Vec<DPresence>,
    pub notification_settings: NotificationSettings,
    pub private_channels: Vec<PrivateChannel>,
    pub consents: Consents,
    pub guild_experiments: Vec<Vec<Option<DGuildExperiment>>>,
    pub user_guild_settings: Vec<UserGuildSetting>,
    pub user_settings_proto: String,
    pub static_client_session_id: String,
    pub guilds: Vec<Guild>,
    pub explicit_content_scan_version: i64,
    pub analytics_token: String,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub experiments: Vec<Vec<i64>>,
    pub user: User,
    pub v: i64,
    pub country_code: String,
    pub friend_suggestion_count: i64,
    pub read_state: Vec<ReadState>,
    pub auth_session_id_hash: String,
    pub user_settings: UserSettings,
    pub sessions: Vec<Session>,
    pub notes: Notes,
    pub geo_ordered_rtc_regions: Vec<String>,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consents {
    pub personalization: Personalization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Personalization {
    pub consented: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DGuildExperiment {
    Integer(i64),
    String(String),
    UnionArray(Vec<TentacledGuildExperiment>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledGuildExperiment {
    FluffyGuildExperiment(FluffyGuildExperiment),
    UnionArray(Vec<StickyGuildExperiment>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyGuildExperiment {
    Integer(i64),
    UnionArray(Vec<IndigoGuildExperiment>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoGuildExperiment {
    PurpleGuildExperiment(PurpleGuildExperiment),
    UnionArray(Vec<Option<IndecentGuildExperiment>>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentGuildExperiment {
    Bool(bool),
    Integer(i64),
    UnionArray(Vec<HilariousGuildExperiment>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousGuildExperiment {
    Integer(i64),
    String(String),
    UnionArray(Vec<AmbitiousGuildExperiment>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousGuildExperiment {
    PurpleGuildExperiment(PurpleGuildExperiment),
    UnionArray(Vec<CunningGuildExperiment>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningGuildExperiment {
    Integer(i64),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleGuildExperiment {
    pub s: i64,
    pub e: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyGuildExperiment {
    pub k: Vec<String>,
    pub b: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notes {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub version: i64,
    #[serde(rename = "type")]
    pub channel_type: i64,
    pub topic: Option<String>,
    pub rate_limit_per_user: Option<i64>,
    pub position: i64,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<String>,
    pub nsfw: Option<bool>,
    pub name: String,
    pub last_message_id: Option<String>,
    pub id: String,
    pub icon_emoji: Option<IconEmoji>,
    pub flags: i64,
    pub user_limit: Option<i64>,
    pub bitrate: Option<i64>,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconEmoji {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    #[serde(rename = "type")]
    pub permission_overwrite_type: i64,
    pub id: String,
    pub deny: String,
    pub allow: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Emoji {
    pub version: i64,
    pub require_colons: bool,
    pub name: String,
    pub managed: bool,
    pub id: String,
    pub available: bool,
    pub animated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub user: UserElement,
    pub roles: Vec<String>,
    pub pending: bool,
    pub nick: Option<String>,
    pub mute: bool,
    pub joined_at: String,
    pub flags: i64,
    pub deaf: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserElement {
    pub username: String,
    pub id: String,
    pub global_name: Option<String>,
    pub display_name: Option<String>,
    pub discriminator: String,
    pub bot: Option<bool>,
    pub avatar: Option<String>,
    pub system: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuildPresence {
    pub user: PartyClass,
    pub status: String,
    pub client_status: ClientStatus,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assets {
    pub large_text: String,
    pub large_image: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyClass {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timestamps {
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub version: i64,
    pub tags: Tags,
    pub position: i64,
    pub permissions: String,
    pub name: String,
    pub mentionable: bool,
    pub managed: bool,
    pub id: String,
    pub hoist: bool,
    pub flags: i64,
    pub color: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    pub bot_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub flags: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DPresence {
    pub user: UserElement,
    pub status: Status,
    pub client_status: ClientStatus,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateChannel {
    #[serde(rename = "type")]
    pub private_channel_type: i64,
    pub recipients: Vec<UserElement>,
    pub recipient_flags: i64,
    pub last_message_id: String,
    pub id: String,
    pub flags: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadState {
    pub mention_count: i64,
    pub last_pin_timestamp: String,
    pub last_message_id: LastMessageId,
    pub id: String,
    pub flags: i64,
    pub last_viewed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LastMessageId {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub user_ignored: bool,
    pub user: UserElement,
    #[serde(rename = "type")]
    pub relationship_type: i64,
    pub since: Option<String>,
    pub is_spam_request: bool,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub status: String,
    pub session_id: String,
    pub client_info: ClientInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientInfo {
    pub version: i64,
    pub os: String,
    pub client: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DUser {
    pub verified: bool,
    pub username: String,
    pub purchased_flags: i64,
    pub pronouns: String,
    pub premium_type: i64,
    pub premium: bool,
    pub mobile: bool,
    pub mfa_enabled: bool,
    pub id: String,
    pub global_name: String,
    pub flags: i64,
    pub email: String,
    pub discriminator: String,
    pub desktop: bool,
    pub bio: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserGuildSetting {
    pub version: i64,
    pub suppress_roles: bool,
    pub suppress_everyone: bool,
    pub notify_highlights: i64,
    pub muted: bool,
    pub mute_scheduled_events: bool,
    pub mobile_push: bool,
    pub message_notifications: i64,
    pub hide_muted_channels: bool,
    pub guild_id: String,
    pub flags: i64,
    pub channel_overrides: Vec<ChannelOverride>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelOverride {
    pub muted: bool,
    pub message_notifications: i64,
    pub flags: i64,
    pub collapsed: bool,
    pub channel_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSettings {
    pub detect_platform_accounts: bool,
    pub animate_stickers: i64,
    pub inline_attachment_media: bool,
    pub status: String,
    pub message_display_compact: bool,
    pub allow_activity_party_privacy_voice_channel: bool,
    pub view_nsfw_guilds: bool,
    pub timezone_offset: i64,
    pub enable_tts_command: bool,
    pub disable_games_tab: bool,
    pub stream_notifications_enabled: bool,
    pub animate_emoji: bool,
    pub friend_source_flags: FriendSourceFlags,
    pub allow_activity_party_privacy_friends: bool,
    pub convert_emoticons: bool,
    pub afk_timeout: i64,
    pub passwordless: bool,
    pub contact_sync_enabled: bool,
    pub gif_auto_play: bool,
    pub native_phone_integration_enabled: bool,
    pub allow_accessibility_detection: bool,
    pub friend_discovery_flags: i64,
    pub show_current_game: bool,
    pub developer_mode: bool,
    pub view_nsfw_commands: bool,
    pub render_reactions: bool,
    pub locale: String,
    pub render_embeds: bool,
    pub inline_embed_media: bool,
    pub default_guilds_restricted: bool,
    pub explicit_content_filter: i64,
    pub theme: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriendSourceFlags {
    pub all: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialize_ready() {
        let value = json!({
            "v": 10,
            "user": {
                "verified": true,
                "username": "test",
                "id": "123",
                "global_name": "Test User",
                "discriminator": "1234"
            },
            "session_id": "123abc",
            "relationships": [],
            "private_channels": [],
            "presences": [],
            "guilds": [],
            "geo_ordered_rtc_regions": ["us-east", "us-west"],
            "session_type": "normal",
            "resume_gateway_url": "wss://gateway.discord.gg",
            "analytics_token": "abc123",
            "auth_session_id_hash": "xyz789",
            "application": {
                "id": "123",
                "flags": 0
            }
        });

        let _result = serde_json::from_value::<Ready>(value).unwrap();
    }
}
