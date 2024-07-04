use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct MfaRequiredSchema {
    pub message: String,
    pub code: i32,
    pub mfa: MfaVerificationSchema, 
}

impl Display for MfaRequiredSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MfaRequired")
            .field("message", &self.message)
            .field("code", &self.code)
            .field("mfa", &self.mfa)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct MfaVerificationSchema {
    pub ticket: String,
    pub methods: Vec<MfaMethod>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct MfaMethod {
    #[serde(rename = "type")]
    pub kind: MfaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes_allowed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MfaType {
    TOTP,
    SMS,
    Backup,
    WebAuthn,
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct MfaVerifySchema {
    pub ticket: String,
    pub mfa_type: MfaType,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaTokenSchema {
    pub token: String,
}
