//! # Common Utility Module for Cortex Examples
//!
//! This module provides shared functionality used across various examples for interacting
//! with the Cortex API. It handles:
//!
//! 1.  **Configuration Setup**: Reads Cortex API endpoint URL and API key from
//!     environment variables (`CORTEX_ENDPOINT` and `CORTEX_API_KEY`) and
//!     initializes a `client::apis::configuration::Configuration` object.
//!
//! 2.  **Analyzer ID Retrieval**: Fetches all analyzer instances from Cortex and
//!     allows finding a specific analyzer's instance ID by its unique name. This
//!     ID is required when creating analyzer jobs.
//!
//! ## Environment Variables
//!
//! Ensure the following environment variables are set before running examples that use this module:
//!
//! -   `CORTEX_ENDPOINT`: The full URL to your Cortex API (e.g., `http://localhost:9000/api`).
//! -   `CORTEX_API_KEY`: Your API key for authenticating with Cortex.
//!
//! ## Usage
//!
//! Examples typically call `common::setup_configuration()` at the beginning to get a
//! `Configuration` object. Then, they might use `common::get_analyzer_id_by_name()`
//! to resolve an analyzer's name (e.g., "AbuseIPDB_1_0") to its operational ID.

use cortex_client::apis::configuration::Configuration;
use std::env;

pub fn setup_configuration() -> Result<Configuration, String> {
    let base_path = env::var("CORTEX_ENDPOINT")
        .map_err(|_| "CORTEX_ENDPOINT environment variable not set. Please set it to your Cortex API URL (e.g., http://localhost:9000/api).".to_string())?;

    let api_key = env::var("CORTEX_API_KEY").map_err(|_| {
        "CORTEX_API_KEY environment variable not set. Please set your Cortex API key.".to_string()
    })?;

    let mut configuration = Configuration::new();
    configuration.base_path = base_path;
    configuration.bearer_access_token = Some(api_key);

    Ok(configuration)
}

pub async fn get_analyzer_id_by_name(
    config: &Configuration,
    analyzer_name_to_find: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!(
        "Fetching all analyzer instances to find ID for '{}'...",
        analyzer_name_to_find
    );

    let find_request = Some(cortex_client::models::AnalyzerFindRequest::default());

    match cortex_client::apis::analyzer_api::find_analyzers(config, find_request).await {
        Ok(analyzer_instances) => {
            // Directly a Vec<Worker>
            for analyzer_instance in analyzer_instances {
                if let Some(name) = &analyzer_instance.name {
                    if name == analyzer_name_to_find {
                        if let Some(id) = analyzer_instance._id {
                            println!(
                                "Found analyzer ID '{}' for name '{}'",
                                id, analyzer_name_to_find
                            );
                            return Ok(Some(id));
                        }
                    }
                }
            }
            println!("Analyzer with name '{}' not found.", analyzer_name_to_find);
            Ok(None)
        }
        Err(e) => {
            eprintln!("Error fetching analyzer instances: {:?}", e);
            Err(Box::new(e))
        }
    }
}
