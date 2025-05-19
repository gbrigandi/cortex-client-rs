//! # Cortex AbuseIPDB Analyzer Example
//!
//! This example demonstrates how to use the Rust client for the Cortex API
//! to submit an IP address for analysis using a specific analyzer instance,
//! in this case, an AbuseIPDB analyzer.
//!
//! ## Functionality
//!
//! 1.  **Configuration**: Sets up the API client configuration using `common::setup_configuration()`,
//!     which reads the Cortex endpoint and API key from environment variables.
//! 2.  **Analyzer Selection**: Specifies the name of the analyzer to run (e.g., "AbuseIPDB_1_0").
//! 3.  **Analyzer ID Retrieval**: Uses `common::get_analyzer_id_by_name()` to find the
//!     actual instance ID of the specified analyzer. This is crucial because Cortex
//!     jobs are run against specific analyzer *instances*.
//! 4.  **Job Creation**: Constructs a `JobCreateRequest` with the IP address to analyze,
//!     data type ("ip"), TLP/PAP levels, and a descriptive message/label.
//! 5.  **Job Submission**: Calls `job_api::create_analyzer_job()` to submit the analysis job.
//! 6.  **Report Fetching**: If the job is submitted successfully and a job ID is returned,
//!     it attempts to fetch the job report using `job_api::get_job_report()`.
//!
//! ## Prerequisites
//!
//! -   A running Cortex instance accessible via the network.
//! -   The "AbuseIPDB" analyzer (or the one specified in `analyzer_name_to_run`)
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
//! cargo run --example abuseipdb_example
use cortex_client::apis::job_api;
use cortex_client::models::JobCreateRequest;

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
            eprintln!("  cargo run --example abuseipdb_example");
            return Err(e.into());
        }
    };

    let analyzer_name_to_run = "AbuseIPDB_1_0"; 

    let ip_to_analyze = "8.8.8.8"; 
    let data_type = "ip";

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
        tlp: Some(2), // Traffic Light Protocol: AMBER (suitable for sharing with trusted partners)
        pap: Some(2), // Permissible Actions Protocol: AMBER
        message: Some(Some(format!(
            "Running {} (instance ID {}) scan from example for IP {}",
            analyzer_name_to_run, analyzer_worker_id, ip_to_analyze
        ))),
        parameters: None, // Add specific analyzer parameters here if needed (e.g., threshold for AbuseIPDB)
        label: Some(Some("abuseipdb_example_scan".to_string())),
        force: Some(false), // Set to true to bypass cache and force a new analysis
        attributes: None,   // Legacy field, prefer direct fields like data, dataType, tlp, etc.
    };

    match job_api::create_analyzer_job(&config, &analyzer_worker_id, job_request).await {
        Ok(job_response) => {
            println!("\nSuccessfully created analyzer job:");
            println!("{:#?}", job_response); 

            if let Some(job_id) = job_response._id {
                println!("\nJob status is Success. Attempting to fetch report directly using get_job_report for job ID: {}", job_id);

                match job_api::get_job_report(&config, &job_id).await {
                    Ok(report_response) => {
                        println!("\nSuccessfully fetched job report:");
                        match report_response {
                            cortex_client::models::JobReportResponse::Object(json_report) => {
                                println!("Report (JSON): {:#?}", json_report);
                            }
                            cortex_client::models::JobReportResponse::JobReportResponseOneOf(
                                status_enum,
                            ) => {
                                println!("Job status from get_job_report: {:?}", status_enum);
                                println!("This is unexpected if the job was marked 'Success' previously.");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("\nError fetching job report with get_job_report: {:?}", e);
                    }
                }
            } else {
                eprintln!("\nJob created, but no job ID was returned in the response. Cannot fetch report.");
            }
        }
        Err(e) => {
            eprintln!("\nError creating analyzer job: {:?}", e);
            eprintln!("Please check:");
            eprintln!(
                "  1. The analyzer ID '{}' is correct and the analyzer is enabled in Cortex.",
                analyzer_worker_id
            );
            eprintln!("  2. Cortex is running and accessible at the configured CORTEX_ENDPOINT.");
            eprintln!("  3. Your CORTEX_API_KEY has the necessary permissions.");
        }
    }

    Ok(())
}
