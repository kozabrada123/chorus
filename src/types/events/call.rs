// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{Snowflake, VoiceState, WebSocketEvent};
use chorus_macros::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Officially Undocumented;
/// Is sent to a client by the server to signify a new call being created;
///
/// Ex: {"t":"CALL_CREATE","s":2,"op":0,"d":{"voice_states":[],"ringing":[],"region":"milan","message_id":"1107187514906775613","embedded_activities":[],"channel_id":"837609115475771392"}}
pub struct CallCreate {
    pub voice_states: Vec<VoiceState>,
    /// Seems like a vec of channel ids
    pub ringing: Vec<String>,
    pub region: String,
    // milan
    pub message_id: Snowflake,
    /// What is this?
    pub embedded_activities: Vec<serde_json::Value>,
    pub channel_id: Snowflake,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, WebSocketEvent)]
/// Officially Undocumented;
/// Updates the client on which calls are ringing, along with a specific call?;
///
/// Ex: {"t":"CALL_UPDATE","s":5,"op":0,"d":{"ringing":["837606544539254834"],"region":"milan","message_id":"1107191540234846308","guild_id":null,"channel_id":"837609115475771392"}}
pub struct CallUpdate {
    /// Seems like a vec of channel ids
    pub ringing: Vec<Snowflake>,
    pub region: String,
    // milan
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
}

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Default,
    Clone,
    PartialEq,
    Eq,
    WebSocketEvent,
    Copy,
    PartialOrd,
    Ord,
)]
/// Officially Undocumented;
/// Deletes a ringing call;
/// Ex: {"t":"CALL_DELETE","s":8,"op":0,"d":{"channel_id":"837609115475771392"}}
pub struct CallDelete {
    pub channel_id: Snowflake,
}

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Default,
    Clone,
    PartialEq,
    Eq,
    WebSocketEvent,
    Copy,
    PartialOrd,
    Ord,
)]
/// Officially Undocumented;
/// See <https://unofficial-discord-docs.vercel.app/gateway/op13>;
///
/// Ex: {"op":13,"d":{"channel_id":"837609115475771392"}}
pub struct CallSync {
    pub channel_id: Snowflake,
}
