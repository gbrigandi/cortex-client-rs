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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkerConfiguration {
    /// Raw JSON string of the worker's specific configuration.
    String(String),
    /// Parsed JSON object of the worker's specific configuration (typically for admin views).
    Object(std::collections::HashMap<String, serde_json::Value>),
}

impl Default for WorkerConfiguration {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

