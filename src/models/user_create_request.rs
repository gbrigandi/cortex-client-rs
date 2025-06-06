/*
 * Cortex API
 *
 * API for Cortex, a powerful observable analysis and active response engine.
 *
 * The version of the OpenAPI document: 3.1.8
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateRequest {
    #[serde(rename = "login")]
    pub login: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Roles>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "preferences", skip_serializing_if = "Option::is_none")]
    pub preferences: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Base64 representation of user avatar image.
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
}

impl UserCreateRequest {
    pub fn new(login: String) -> UserCreateRequest {
        UserCreateRequest {
            login,
            name: None,
            roles: None,
            organization: None,
            password: None,
            status: None,
            preferences: None,
            avatar: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Roles {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "analyze")]
    Analyze,
    #[serde(rename = "orgadmin")]
    Orgadmin,
    #[serde(rename = "superadmin")]
    Superadmin,
}

impl Default for Roles {
    fn default() -> Roles {
        Self::Read
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "Locked")]
    Locked,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ok
    }
}

