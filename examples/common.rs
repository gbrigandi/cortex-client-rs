//! # Common Utility Module for Cortex Examples
//!
//! This module provides shared functionality used across various examples for interacting
//! with the Cortex API. It handles:
//!
//! 1.  **Configuration Setup**: Reads Cortex API endpoint URL and API key from
//!     environment variables (`CORTEX_ENDPOINT` and `CORTEX_API_KEY`) and
//!     initializes a `cortex_client::apis::configuration::Configuration` object.
//!
//! 2.  **Analyzer ID Retrieval**: Fetches all analyzer instances from Cortex and
//!     allows finding a specific analyzer's instance ID by its unique name. This
//!     ID is required when creating analyzer jobs.
//!
//! 3.  **Job Execution and Report Waiting**: Provides a helper function
//!     `run_job_and_wait_for_report` that:
//!     *   Submits an analyzer job.
//!     *   Polls the job status with a configurable number of retries and delay.
//!     *   Fetches and returns the job report if the job completes successfully.
//!     *   Handles job failures or timeouts gracefully.
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
//! `Configuration` object. They might use `common::get_analyzer_id_by_name()`
//! to resolve an analyzer's name to its ID, and `common::run_job_and_wait_for_report()`
//! to execute an analysis and retrieve its results.

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

pub async fn run_job_and_wait_for_report(
    config: &cortex_client::apis::configuration::Configuration,
    analyzer_worker_id: &str,
    job_request: cortex_client::models::JobCreateRequest,
    analyzer_name_for_log: &str, // For clearer log messages
    observable_for_log: &str,    // For clearer log messages
) -> Result<cortex_client::models::JobReportResponse, Box<dyn std::error::Error>> {
    use cortex_client::apis::job_api;
    use std::time::Duration;

    println!(
        "Submitting job to analyzer '{}' (ID: '{}') for observable: {}...",
        analyzer_name_for_log, analyzer_worker_id, observable_for_log
    );

    match job_api::create_analyzer_job(config, analyzer_worker_id, job_request).await {
        Ok(job_response) => {
            println!(
                "Successfully created job. Job ID: {}, Status: {:?}",
                job_response._id.as_ref().unwrap_or(&"unknown".to_string()),
                job_response.status
            );

            if let Some(job_id) = job_response._id {
                println!(
                    "\nAttempting to fetch report for job ID: {} with retries...",
                    job_id
                );

                let max_retries = 3;
                let retry_delay = Duration::from_secs(15);

                for attempt in 1..=max_retries {
                    println!(
                        "\nAttempt {} of {} to check job status...",
                        attempt, max_retries
                    );
                    match job_api::get_job_by_id(config, &job_id).await {
                        Ok(job_details) => {
                            println!("Current job status: {:?}", job_details.status);
                            match job_details.status {
                                Some(cortex_client::models::job::Status::Success) => {
                                    println!(
                                        "Job status is Success. Attempting to fetch report..."
                                    );
                                    match job_api::get_job_report(config, &job_id).await {
                                        Ok(report_response) => {
                                            println!("\n✅ Successfully fetched job report!");
                                            return Ok(report_response);
                                        }
                                        Err(e) => {
                                            let err_msg = format!("Error fetching job report even though status was Success: {:?}", e);
                                            eprintln!("{}", err_msg);
                                            // Decide if to break or retry. For now, break on report fetch error.
                                            return Err(err_msg.into());
                                        }
                                    }
                                }
                                Some(cortex_client::models::job::Status::Failure) => {
                                    let err_msg = format!(
                                        "Job failed. Error message: {:?}",
                                        job_details.error_message.unwrap_or_else(|| Some(
                                            "No error message provided.".to_string()
                                        ))
                                    );
                                    eprintln!("{}", err_msg);
                                    return Err(err_msg.into());
                                }
                                Some(cortex_client::models::job::Status::InProgress)
                                | Some(cortex_client::models::job::Status::Waiting) => {
                                    if attempt < max_retries {
                                        println!("Job is still {:?}. Waiting for {} seconds before next attempt...", job_details.status.unwrap(), retry_delay.as_secs());
                                        tokio::time::sleep(retry_delay).await;
                                    } else {
                                        let info_msg = format!("Job did not complete (still {:?}) after {} attempts. Job ID: {}", job_details.status.unwrap(), max_retries, job_id);
                                        println!("{}", info_msg);
                                        // Return an error indicating timeout after retries
                                        return Err(info_msg.into());
                                    }
                                }
                                Some(cortex_client::models::job::Status::Deleted) => {
                                    let err_msg = format!(
                                        "Job has been deleted. Cannot fetch report. Job ID: {}",
                                        job_id
                                    );
                                    eprintln!("{}", err_msg);
                                    return Err(err_msg.into());
                                }
                                None => {
                                    let warn_msg = format!("Job status is unknown. Cannot determine if report is ready. Job ID: {}", job_id);
                                    eprintln!("{}", warn_msg);
                                    if attempt < max_retries {
                                        tokio::time::sleep(retry_delay).await;
                                    } else {
                                        let info_msg = format!("Job status remained unknown after {} attempts. Job ID: {}", max_retries, job_id);
                                        println!("{}", info_msg);
                                        return Err(info_msg.into());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "\nError fetching job details on attempt {}: {:?}",
                                attempt, e
                            );
                            if attempt == max_retries {
                                let err_msg = format!("Could not fetch job details after {} attempts for job ID: {}. Error: {:?}", max_retries, job_id, e);
                                eprintln!("{}", err_msg);
                                return Err(err_msg.into());
                            } else {
                                tokio::time::sleep(retry_delay).await;
                            }
                        }
                    }
                }
                let final_err_msg = format!(
                    "\nCould not retrieve job report for job ID {} after {} attempts.",
                    job_id, max_retries
                );
                eprintln!("{}", final_err_msg);
                Err(final_err_msg.into())
            } else {
                let err_msg = "\nJob created, but no job ID was returned in the response. Cannot fetch report.".to_string();
                eprintln!("{}", err_msg);
                Err(err_msg.into())
            }
        }
        Err(e) => {
            let err_msg = format!(
                "\nError creating analyzer job for '{}' on '{}': {:?}",
                analyzer_name_for_log, observable_for_log, e
            );
            eprintln!("{}", err_msg);
            Err(err_msg.into())
        }
    }
}
