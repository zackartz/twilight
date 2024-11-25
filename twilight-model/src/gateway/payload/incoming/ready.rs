use crate::{
    gateway::{presence::Activity, ShardId},
    guild::Guild,
    oauth::PartialApplication,
    user::CurrentUser,
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

/// Session type for the connection
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionType {
    /// Normal session
    Normal,
}

/// Client information about the session
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClientInfo {
    /// The client's version
    pub version: u32,
    /// The client's operating system
    pub os: String,
    /// The client type (e.g., "web")
    pub client: String,
}

/// Information about a session
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    /// Session ID
    pub session_id: String,
    /// Session status (e.g., "online", "dnd")
    pub status: String,
    /// Client information
    pub client_info: ClientInfo,
    /// List of activities for the session
    pub activities: Vec<Activity>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ready {
    /// Current application info
    pub application: PartialApplication,
    /// Array of guild objects
    pub guilds: Vec<Guild>,
    /// Used for resuming connections
    pub resume_gateway_url: String,
    /// Session ID
    pub session_id: String,
    /// Gateway version
    #[serde(rename = "v")]
    pub version: u8,
    /// The shard information associated with this session, if sent when identifying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardId>,
    /// Current user
    pub user: CurrentUser,
    /// Contains id and flags
    pub analytics_token: String,
    /// Connected user settings
    #[serde(skip)]
    pub user_settings: Option<UserSettings>,
    /// List of servers for RTC regions
    #[serde(default)]
    pub geo_ordered_rtc_regions: Vec<String>,
    /// Session type
    pub session_type: SessionType,
    /// List of read states
    #[serde(default)]
    pub read_state: Vec<ReadState>,
    /// List of user sessions
    #[serde(default)]
    pub sessions: Vec<Session>,
    /// Auth session ID hash
    pub auth_session_id_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserSettings;

/// Read state for a channel
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadState {
    /// ID of the channel
    pub id: String,
    /// Last message ID that was read
    pub last_message_id: String,
    /// Timestamp of the last pin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,
    /// Mention count
    pub mention_count: u64,
    /// Read state flags
    pub flags: u64,
    /// Last viewed timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed: Option<u64>,
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
