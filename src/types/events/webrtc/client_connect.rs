use crate::types::{Snowflake, WebSocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Sent when another user connects to the voice server.
///
/// Contains the user id and "flags".
///
/// Not documented anywhere, if you know what this is, please reach out
///
/// {"op":18,"d":{"user_id":"1234567890","flags":2}}
pub struct VoiceClientConnectFlags {
    pub user_id: Snowflake,
    // Likely some sort of bitflags
    pub flags: u8,
}

impl WebSocketEvent for VoiceClientConnectFlags {}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Sent when another user connects to the voice server.
///
/// Contains the user id and "platform".
///
/// Not documented anywhere, if you know what this is, please reach out
///
/// {"op":20,"d":{"user_id":"1234567890","platform":0}}
pub struct VoiceClientConnectPlatform {
    pub user_id: Snowflake,
    // Likely an enum
    pub platform: u8,
}

impl WebSocketEvent for VoiceClientConnectPlatform {}
