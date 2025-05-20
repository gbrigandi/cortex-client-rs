//! # Cortex Wait for Report Example
//!
//! This example demonstrates how to use the Rust client for the Cortex API
//! to submit an IP address for analysis using a specific analyzer instance.
//! It leverages a common helper function to submit the job and then wait
//! (with retries) for the analysis report to be generated.
//!
//! ## Functionality
//!
//! 1.  **Configuration**: Sets up the API client configuration using `common::setup_configuration()`,
//!     which reads the Cortex endpoint and API key from environment variables.
//! 2.  **Analyzer Selection**: Specifies the name of the analyzer to run (e.g., "AbuseIPDB_1_0").
//! 3.  **Analyzer ID Retrieval**: Uses `common::get_analyzer_id_by_name()` to find the
//!     actual instance ID of the specified analyzer.
//! 4.  **Job Request Construction**: Constructs a `JobCreateRequest` with the IP address to analyze,
//!     data type ("ip"), TLP/PAP levels, and a descriptive message/label.
//! 5.  **Job Submission and Report Fetching**: Calls the `common::run_job_and_wait_for_report()`
//!     helper function. This function:
//!     *   Submits the job to the specified analyzer.
//!     *   Polls the job status with retries (e.g., up to 3 attempts with a 15-second delay).
//!     *   If the job status becomes `Success`, it fetches and returns the `JobReportResponse`.
//!     *   Handles `Failure` status or if the job doesn't complete within retries by returning an error.
//! 6.  **Display Report**: If successful, the example prints key details from the report,
//!     such as summary and taxonomies, and then the full JSON report.
//!
//! ## Prerequisites
//!
//! -   A running Cortex instance accessible via the network.
//! -   The analyzer specified in `analyzer_name_to_run` (e.g., "AbuseIPDB_1_0")
//!     must be enabled and configured in your Cortex instance.
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
//! cargo run --example wait_report_example
//! ```
use cortex_client::apis::job_api;
use cortex_client::models::JobCreateRequest;
use std::time::Duration;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match common::setup_configuration() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            eprintln!(
                "Please ensure CORTEX_ENDPOINT and CORTEX_API_KEY environment variables are set."
            );
            eprintln!("Example usage:");
            eprintln!("  export CORTEX_ENDPOINT=\"http://localhost:9000/api\"");
            eprintln!("  export CORTEX_API_KEY=\"your_api_key_here\"");
            eprintln!("  cargo run --example wait_report_example");
            return Err(e.into());
        }
    };

    // You can change this to any analyzer that's available in your Cortex instance
    let analyzer_name_to_run = "AbuseIPDB_1_0";

    // The observable to analyze
    let ip_to_analyze = "8.8.8.8";
    let data_type = "ip";

    println!("Looking for analyzer: {}", analyzer_name_to_run);
    let analyzer_worker_id = match common::get_analyzer_id_by_name(&config, analyzer_name_to_run)
        .await
    {
        Ok(Some(id)) => id,
        Ok(None) => {
            eprintln!("Could not find an analyzer instance named '{}'. Please check the name and ensure the analyzer is enabled in Cortex.", analyzer_name_to_run);
            eprintln!("You can use the 'list_analyzers' example to see available analyzer names and their instance IDs.");
            return Ok(()); // Or return an error
        }
        Err(e) => {
            eprintln!(
                "Error trying to get analyzer ID for '{}': {}",
                analyzer_name_to_run, e
            );
            return Err(e);
        }
    };

    println!(
        "Attempting to run analyzer instance ID '{}' (resolved from name '{}') on IP: {}",
        analyzer_worker_id, analyzer_name_to_run, ip_to_analyze
    );

    let job_request = JobCreateRequest {
        data: Some(ip_to_analyze.to_string()),
        data_type: Some(data_type.to_string()),
        tlp: Some(2), // Traffic Light Protocol: AMBER
        pap: Some(2), // Permissible Actions Protocol: AMBER
        message: Some(Some(format!(
            "Running {} (instance ID {}) scan from wait_report example for IP {}",
            analyzer_name_to_run, analyzer_worker_id, ip_to_analyze
        ))),
        parameters: None,
        label: Some(Some("wait_report_example_scan".to_string())),
        force: Some(true), // Force a new analysis even if cached results exist
        attributes: None,
    };

    match common::run_job_and_wait_for_report(
        &config,
        &analyzer_worker_id,
        job_request,
        analyzer_name_to_run,
        ip_to_analyze,
    )
    .await
    {
        Ok(report_response) => {
            // The common function already prints "✅ Successfully fetched job report!"
            println!("\nReport details from wait_report_example:");
            match report_response {
                cortex_client::models::JobReportResponse::Object(json_report) => {
                    // Try to extract and display some key information
                    if let Some(summary) = json_report.get("summary") {
                        println!("Summary: {}", summary);
                    }
                    if let Some(taxonomies) = json_report.get("taxonomies") {
                        if let Some(tax_array) = taxonomies.as_array() {
                            println!("\nTaxonomies:");
                            for taxonomy in tax_array {
                                if let (
                                    Some(level),
                                    Some(namespace),
                                    Some(predicate),
                                    Some(value),
                                ) = (
                                    taxonomy.get("level").and_then(|v| v.as_str()),
                                    taxonomy.get("namespace").and_then(|v| v.as_str()),
                                    taxonomy.get("predicate").and_then(|v| v.as_str()),
                                    taxonomy.get("value").and_then(|v| v.as_str()),
                                ) {
                                    println!(
                                        "  - {}: {}:{}={}",
                                        level, namespace, predicate, value
                                    );
                                }
                            }
                        }
                    }
                    println!("\nFull report (JSON):");
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&json_report)
                            .unwrap_or_else(|_| json_report.to_string())
                    );
                }
                cortex_client::models::JobReportResponse::JobReportResponseOneOf(
                    status_enum,
                ) => {
                    // This case should ideally be handled by the common function returning an Err
                    // if the job didn't result in a successful object report.
                    eprintln!(
                        "Received non-object report status from common function: {:?}",
                        status_enum
                    );
                    eprintln!("This might indicate the job did not complete successfully or the report is not in the expected format.");
                }
            }
        }
        Err(e) => {
            eprintln!(
                "\nError in run_job_and_wait_for_report for analyzer '{}' on IP '{}': {:?}",
                analyzer_name_to_run, ip_to_analyze, e
            );
            eprintln!("Please check the logs from the common function for more details.");
            eprintln!("Also, ensure:");
            eprintln!(
                "  1. The analyzer ID '{}' (for '{}') is correct and the analyzer is enabled.",
                analyzer_worker_id, analyzer_name_to_run
            );
            eprintln!("  2. Cortex is running and accessible at the configured CORTEX_ENDPOINT.");
            eprintln!("  3. Your CORTEX_API_KEY has the necessary permissions.");
        }
    }

    Ok(())
}
