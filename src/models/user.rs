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
pub struct User {
    /// User ID (same as login).
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    /// Full name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Roles>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    /// User preferences as a JSON object.
    #[serde(rename = "preferences", skip_serializing_if = "Option::is_none")]
    pub preferences: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Indicates if the user has an API key.
    #[serde(rename = "hasKey", skip_serializing_if = "Option::is_none")]
    pub has_key: Option<bool>,
    /// Indicates if the user has a password set.
    #[serde(rename = "hasPassword", skip_serializing_if = "Option::is_none")]
    pub has_password: Option<bool>,
}

impl User {
    pub fn new() -> User {
        User {
            _id: None,
            login: None,
            name: None,
            roles: None,
            status: None,
            avatar: None,
            preferences: None,
            organization: None,
            has_key: None,
            has_password: None,
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

