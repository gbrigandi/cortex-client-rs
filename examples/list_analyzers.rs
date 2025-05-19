//! # Cortex List Analyzers Example
//!
//! This example demonstrates how to use the Rust client for the Cortex API
//! to fetch and list all available analyzer instances configured in your Cortex setup.
//!
//! ## Functionality
//!
//! 1.  **Configuration**: Sets up the API client configuration using `common::setup_configuration()`,
//!     which relies on `CORTEX_ENDPOINT` and `CORTEX_API_KEY` environment variables.
//! 2.  **Fetch Analyzers**: Calls `analyzer_api::find_analyzers()` to retrieve a list
//!     of all analyzer instances.
//! 3.  **Display Information**: Iterates through the fetched analyzer instances and prints
//!     key details for each, including:
//!     *   Name (e.g., "AbuseIPDB_1_0", "VirusTotal_GetReport_3_0")
//!     *   Instance ID (`_id`): This is the crucial ID required to run a job with a specific analyzer.
//!     *   Worker Definition ID
//!     *   Type (Analyzer/Responder)
//!
//! ## Importance of Instance ID
//!
//! When you want to run an analysis (i.e., create an analyzer job), you need to provide
//! the specific `Instance ID` (referred to as `analyzer_worker_id` or similar in other examples)
//! of the analyzer you wish to use. This example helps you discover these IDs by listing
//! all configured analyzer instances and their names.
//!
//! ## Prerequisites
//!
//! -   A running Cortex instance.
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
//! cargo run --example list_analyzers
//! ```
//!
//! The output will be a list of analyzer instances. Look for the analyzer you intend to use
//! (e.g., an AbuseIPDB analyzer) and note its `Instance ID (_id)`. This ID can then be
//! used in other examples or your own applications to run jobs with that analyzer.
use cortex_client::apis::analyzer_api;
mod common; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match common::setup_configuration() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            return Err(e.into());
        }
    };

    println!("Fetching list of all analyzer instances...");

    let find_request = Some(cortex_client::models::AnalyzerFindRequest::default());

    match analyzer_api::find_analyzers(&config, find_request).await {
        Ok(analyzers) => {
            println!("\nSuccessfully fetched analyzer instances:");
            if analyzers.is_empty() {
                println!("No analyzer instances found in your Cortex organization.");
            } else {
                println!("--------------------------------------------------");
                println!("Available Analyzer Instances:");
                println!("--------------------------------------------------");
                for worker_instance in &analyzers {
                    // Iterate over the direct vector
                    println!(
                        "  Name:                 {}",
                        worker_instance.name.as_deref().unwrap_or("N/A")
                    );
                    println!(
                        "  Instance ID (_id):    {}", // THIS IS WHAT YOU NEED
                        worker_instance._id.as_deref().unwrap_or("MISSING_ID")
                    );
                    println!(
                        "  Definition ID:        {}",
                        worker_instance
                            .worker_definition_id
                            .as_deref()
                            .unwrap_or("N/A")
                    );
                    println!(
                        "  Type:                 {:?}",
                        worker_instance
                            .r#type
                            .unwrap_or(cortex_client::models::worker::Type::Analyzer) // Provide a default if None
                    );
                    println!("  Enabled (Implicitly): If listed, it's an instance. Check Cortex UI for explicit enabled/disabled status if unsure.");
                    println!("  ---");
                }
                println!("\nTotal analyzer instances found: {}", analyzers.len());
                println!("--------------------------------------------------");
                println!("\nLook for your AbuseIPDB analyzer in the list above and use its 'Instance ID (_id)'");
                println!(
                    "in your abuseipdb_example.rs file for the 'analyzer_worker_id' variable."
                );
            }
        }
        Err(e) => {
            eprintln!("\nError fetching analyzer instances: {:?}", e);
            eprintln!("Please check your Cortex connection and API key permissions.");
            if let cortex_client::apis::Error::Serde(serde_err) = e {
                eprintln!("Serde error details: {}", serde_err);
            }
        }
    }

    Ok(())
}
