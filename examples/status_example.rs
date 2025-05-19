//! # Cortex API Status Example
//!
//! This example demonstrates how to use the Rust client for the Cortex API
//! to fetch and display general status information.
//!
//! ## Functionality
//!
//! 1.  **Configuration**: Sets up the API client configuration using `common::setup_configuration()`.
//! 2.  **Fetch Status**: Calls `status_api::get_status()` to retrieve general system status
//!     and configuration information from the `/status` endpoint.
//! 3.  **Health Status (Note)**: The call to `status_api::get_health_status()` (for `/status/health`)
//!     has been commented out in this example. This endpoint may not be available in all
//!     Cortex versions or configurations, potentially leading to a 404 Not Found error.
//!     The OpenAPI specification itself notes this endpoint as a "TODO".
//!
//! ## Prerequisites
//!
//! -   A running Cortex instance accessible via the network.
//! -   Environment variables `CORTEX_ENDPOINT` and `CORTEX_API_KEY` must be set.
//!
//! ## Usage
//!
//! ```sh
//! # Set your Cortex API endpoint and key
//! export CORTEX_ENDPOINT="http://your-cortex-host:9000/api"
//! export CORTEX_API_KEY="your_cortex_api_key"
//!
//! # Run the example
//! cargo run --example status_example
//! ```
use cortex_client::apis::status_api;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup configuration using the common helper
    let config = match common::setup_configuration() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            eprintln!("Please ensure CORTEX_ENDPOINT and CORTEX_API_KEY environment variables are set.");
            eprintln!("Example usage:");
            eprintln!("  export CORTEX_ENDPOINT=\"http://localhost:9000/api\"");
            eprintln!("  export CORTEX_API_KEY=\"your_api_key_here\"");
            eprintln!("  cargo run --example status_example");
            return Err(e.into());
        }
    };

    println!("Fetching API status (/status)...");
    match status_api::get_status(&config).await {
        Ok(status_response) => {
            println!("Successfully fetched API status:");
            println!("{:#?}", status_response);

            if let Some(versions) = status_response.versions {
                println!("\nVersions:");
                println!("  Cortex: {}", versions.cortex.as_deref().unwrap_or("N/A"));
                println!("  Elastic4Play: {}", versions.elastic4_play.as_deref().unwrap_or("N/A"));
            }
            if let Some(api_config) = status_response.config {
                 println!("\nAPI Configuration:");
                 println!("  Auth Type: {:?}", api_config.auth_type);
                 println!("  Capabilities: {:?}", api_config.capabilities.unwrap_or_default());
            }
        }
        Err(e) => {
            eprintln!("Error fetching API status: {:?}", e);
        }
    }

    Ok(())
}
