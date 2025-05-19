# Cortex API Client for Rust

Cortex is an open-source, powerful observable analysis and active response engine. It allows security professionals and SOC analysts to analyze observables like IPs, URLs, domains, files, and more, using a wide array of integrated analyzers. It also enables automated responses to threats through its responder capabilities.

The Cortex API provides a programmatic interface to its rich set of features, offering several advantages:

*   **Automation:** Automate the submission of observables for analysis, streamlining workflows and reducing manual effort.
*   **Integration:** Seamlessly integrate Cortex's analysis capabilities into existing security tools and platforms (SIEMs, SOARs, TIPs, etc.).
*   **Scalability:** Handle large volumes of observables and analysis requests programmatically.
*   **Customization:** Build custom scripts and tools that leverage Cortex's backend for tailored security operations.
*   **Efficiency:** Speed up incident response by getting analysis results quickly and consistently.

## Adding the Crate to Your Project

To integrate the `cortex-client-rs` into your Rust project, add it as a dependency in your `Cargo.toml` file.

```toml
[dependencies]
cortex-client-rs = "0.1" 
```
## Running The Examples from this Repository

This repository includes several runnable examples in the `examples/` directory. To run them:

1.  **Set Environment Variables:** Ensure your Cortex instance is running and accessible. You'll need to set the following environment variables:
    *   `CORTEX_ENDPOINT`: The full URL to your Cortex API (e.g., `http://localhost:9000/api`).
    *   `CORTEX_API_KEY`: Your API key for authenticating with Cortex.

    Example:
    ```sh
    export CORTEX_ENDPOINT="http://your-cortex-host:9000/api"
    export CORTEX_API_KEY="your_cortex_api_key"
    ```

2.  **Run an Example:** Use `cargo run --example <example_name>`. For instance, to run the `abuseipdb_example`:
    ```sh
    cargo run --example abuseipdb_example
    ```
    Replace `abuseipdb_example` with the name of any other example file in the `examples/` directory (e.g., `list_analyzers`, `status_example`).

## Rust Usage Examples

Below are examples of how to use the generated Rust client to interact with the Cortex API, focusing on running analyzers. These examples are structured similarly to the runnable files found in the `examples/` directory of this repository.

**Note:**
*   These examples assume the client has been generated and is available as the `client` crate.
*   They rely on a helper module, typically named `common.rs` (as seen in the `examples/` directory), which should provide:
    *   `setup_configuration()`: A function to create a `client::apis::configuration::Configuration` object, usually by reading `CORTEX_ENDPOINT` and `CORTEX_API_KEY` from environment variables.
    *   `get_analyzer_id_by_name(config: &Configuration, analyzer_name: &str) -> Result<Option<String>, Box<dyn std::error::Error>>`: A function to find an analyzer's instance ID given its name.
*   Ensure your Cortex instance is running and accessible, and your API key is valid.
*   Error handling is included for clarity.

### 1. Analyze an IP Address

This example demonstrates how to submit an IP address to an analyzer like VirusTotal. It assumes you have a `common.rs` module for setup.

```rust
// main.rs (or your example file)
use cortex_client::apis::job_api;
use cortex_client::models::JobCreateRequest;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match common::setup_configuration() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            eprintln!("Please ensure CORTEX_ENDPOINT and CORTEX_API_KEY environment variables are set.");
            return Err(e.into());
        }
    };

    let analyzer_name_to_run = "VirusTotal_GetReport_3_1"; // Example analyzer name
    let ip_to_analyze = "8.8.8.8";
    let data_type = "ip";

    let analyzer_worker_id = match common::get_analyzer_id_by_name(&config, analyzer_name_to_run).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            eprintln!("Could not find an analyzer instance named '{}'. Ensure it's enabled in Cortex.", analyzer_name_to_run);
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error getting analyzer ID for '{}': {}", analyzer_name_to_run, e);
            return Err(e);
        }
    };

    println!(
        "Attempting to run analyzer '{}' (ID: '{}') on IP: {}",
        analyzer_name_to_run, analyzer_worker_id, ip_to_analyze
    );

    let job_create_request = JobCreateRequest {
        data: Some(ip_to_analyze.to_string()),
        data_type: Some(data_type.to_string()),
        tlp: Some(2), // Traffic Light Protocol: AMBER
        pap: Some(2), // Permissible Actions Protocol: AMBER
        message: Some(Some(format!(
            "README example: Analyzing IP {} with {}",
            ip_to_analyze, analyzer_name_to_run
        ))),
        parameters: None,
        label: Some(Some(format!("readme_ip_analysis_{}", ip_to_analyze))),
        force: Some(false),
        attributes: None,
    };

    match job_api::create_analyzer_job(&config, &analyzer_worker_id, job_create_request).await {
        Ok(job_details) => {
            println!(
                "Successfully submitted job for {} '{}' with analyzer '{}'. Job ID: {}",
                data_type,
                ip_to_analyze,
                analyzer_name_to_run,
                job_details._id.unwrap_or_default()
            );
            println!("Job details: {:#?}", job_details);
        }
        Err(e) => {
            eprintln!("Error submitting IP analysis job: {:?}", e);
            eprintln!("Please check if analyzer '{}' (ID: '{}') is correctly configured and enabled.", analyzer_name_to_run, analyzer_worker_id);
        }
    }
    Ok(())
}
```

### 2. Analyze a File Hash

This example shows how to submit a file hash (e.g., MD5, SHA256) for analysis using an analyzer like HybridAnalysis.

```rust
use cortex_client::apis::job_api;
use cortex_client::models::JobCreateRequest;

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

    let analyzer_name_to_run = "HybridAnalysis_GetReport_1_0"; 
    let file_hash_to_analyze = "d41d8cd98f00b204e9800998ecf8427e"; /
    let data_type = "hash"; 

    let analyzer_worker_id = match common::get_analyzer_id_by_name(&config, analyzer_name_to_run).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            eprintln!("Could not find an analyzer instance named '{}'.", analyzer_name_to_run);
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error getting analyzer ID for '{}': {}", analyzer_name_to_run, e);
            return Err(e);
        }
    };

    println!(
        "Attempting to run analyzer '{}' (ID: '{}') on hash: {}",
        analyzer_name_to_run, analyzer_worker_id, file_hash_to_analyze
    );

    let job_create_request = JobCreateRequest {
        data: Some(file_hash_to_analyze.to_string()),
        data_type: Some(data_type.to_string()),
        tlp: Some(2),
        pap: Some(2),
        message: Some(Some(format!(
            "README example: Analyzing file hash {} with {}",
            file_hash_to_analyze, analyzer_name_to_run
        ))),
        parameters: None,
        label: Some(Some(format!("readme_hash_analysis_{}", file_hash_to_analyze))),
        force: Some(false),
        attributes: None,
    };

    match job_api::create_analyzer_job(&config, &analyzer_worker_id, job_create_request).await {
        Ok(job_details) => {
            println!(
                "Successfully submitted job for {} '{}' with analyzer '{}'. Job ID: {}",
                data_type,
                file_hash_to_analyze,
                analyzer_name_to_run,
                job_details._id.unwrap_or_default()
            );
            println!("Job details: {:#?}", job_details);
        }
        Err(e) => {
            eprintln!("Error submitting file hash analysis job: {:?}", e);
        }
    }
    Ok(())
}
```
**Note on analyzing files:** To analyze an actual file (not just its hash), you would typically set `dataType: "file"` and send the file content as a multipart/form-data request. The `JobCreateRequest` schema anticipates an `attachment` part for this. Your Rust HTTP client library would need to support multipart uploads. The specifics of constructing such a request depend on the generated client's capabilities for handling file uploads.

### 3. Analyze a Domain with Custom Parameters

Some analyzers accept specific parameters. This example shows submitting a domain with custom parameters, using `serde_json::json` to construct them.

```rust
use cortex_client::apis::job_api;
use cortex_client::models::JobCreateRequest;
use serde_json::json; 

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

    let analyzer_name_to_run = "DomainToolsIris_Investigate_1_0"; 
    let domain_to_analyze = "example.com";
    let data_type = "domain";

    let analyzer_worker_id = match common::get_analyzer_id_by_name(&config, analyzer_name_to_run).await {
        Ok(Some(id)) => id,
        Ok(None) => {
            eprintln!("Could not find an analyzer instance named '{}'.", analyzer_name_to_run);
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error getting analyzer ID for '{}': {}", analyzer_name_to_run, e);
            return Err(e);
        }
    };

    println!(
        "Attempting to run analyzer '{}' (ID: '{}') on domain: {}",
        analyzer_name_to_run, analyzer_worker_id, domain_to_analyze
    );

    let custom_params = json!({
        "include_subdomains": true,
        "max_history_days": 90
    });

    let job_create_request = JobCreateRequest {
        data: Some(domain_to_analyze.to_string()),
        data_type: Some(data_type.to_string()),
        tlp: Some(1), // Traffic Light Protocol: GREEN
        pap: Some(1), // Permissible Actions Protocol: GREEN
        message: Some(Some(format!(
            "README example: Analyzing domain {} with {} and custom parameters",
            domain_to_analyze, analyzer_name_to_run
        ))),
        parameters: Some(custom_params),
        label: Some(Some(format!("readme_domain_analysis_{}", domain_to_analyze))),
        force: Some(false),
        attributes: None,
    };

    match job_api::create_analyzer_job(&config, &analyzer_worker_id, job_create_request).await {
        Ok(job_details) => {
            println!(
                "Successfully submitted job for {} '{}' with analyzer '{}'. Job ID: {}",
                data_type,
                domain_to_analyze,
                analyzer_name_to_run,
                job_details._id.unwrap_or_default()
            );
            println!("Job details: {:#?}", job_details);
        }
        Err(e) => {
            eprintln!("Error submitting domain analysis job with parameters: {:?}", e);
        }
    }
    Ok(())
}
```

After submitting a job, you would typically use the returned Job ID to poll its status using `JobApi::get_job_by_id` and, once completed successfully, retrieve the full report using `JobApi::get_job_report`. You can also use `JobApi::wait_job_report` to long-poll for the result.

## Documentation for API Endpoints

All URIs are relative to */api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AnalyzerApi* | [**create_analyzer_from_definition**](docs/AnalyzerApi.md#create_analyzer_from_definition) | **POST** /analyzer/definition/{analyzerDefinitionId} | Create a new analyzer instance from a definition
*AnalyzerApi* | [**delete_analyzer**](docs/AnalyzerApi.md#delete_analyzer) | **DELETE** /analyzer/{analyzerId} | Delete an analyzer
*AnalyzerApi* | [**find_analyzers**](docs/AnalyzerApi.md#find_analyzers) | **POST** /analyzer/_search | Find/Search analyzers
*AnalyzerApi* | [**get_analyzer_by_id**](docs/AnalyzerApi.md#get_analyzer_by_id) | **GET** /analyzer/{analyzerId} | Get a specific analyzer by ID
*AnalyzerApi* | [**list_analyzer_definitions**](docs/AnalyzerApi.md#list_analyzer_definitions) | **GET** /analyzer/definition | List all available analyzer definitions
*AnalyzerApi* | [**list_analyzers_for_type**](docs/AnalyzerApi.md#list_analyzers_for_type) | **GET** /analyzer/type/{dataType} | List analyzers available for a specific data type
*AnalyzerApi* | [**scan_analyzer_definitions**](docs/AnalyzerApi.md#scan_analyzer_definitions) | **POST** /analyzer/scan | Trigger a rescan of analyzer definitions
*AnalyzerApi* | [**update_analyzer**](docs/AnalyzerApi.md#update_analyzer) | **PUT** /analyzer/{analyzerId} | Update an analyzer
*AnalyzerConfigApi* | [**get_analyzer_configuration**](docs/AnalyzerConfigApi.md#get_analyzer_configuration) | **GET** /analyzer/config/{analyzerConfigName} | Get a specific analyzer configuration
*AnalyzerConfigApi* | [**list_analyzer_configurations**](docs/AnalyzerConfigApi.md#list_analyzer_configurations) | **GET** /analyzer/config | List all analyzer configurations for the user
*AnalyzerConfigApi* | [**update_analyzer_configuration**](docs/AnalyzerConfigApi.md#update_analyzer_configuration) | **PUT** /analyzer/config/{analyzerConfigName} | Update or create an analyzer configuration
*ArtifactApi* | [**list_job_artifacts**](docs/ArtifactApi.md#list_job_artifacts) | **POST** /job/{jobId}/artifacts/_search | List/Search artifacts for a job
*AttachmentApi* | [**download_attachment**](docs/AttachmentApi.md#download_attachment) | **GET** /attachment/{hash} | Download an attachment in plain format
*AttachmentApi* | [**download_attachment_zip**](docs/AttachmentApi.md#download_attachment_zip) | **GET** /attachment/{hash}/zip | Download an attachment in a password-protected zip file
*AuthenticationApi* | [**login**](docs/AuthenticationApi.md#login) | **POST** /login | Log in to Cortex
*AuthenticationApi* | [**logout**](docs/AuthenticationApi.md#logout) | **POST** /logout | Log out from Cortex
*AuthenticationApi* | [**sso_login**](docs/AuthenticationApi.md#sso_login) | **GET** /ssoLogin | Initiate or handle SSO login
*DbListApi* | [**add_db_list_item**](docs/DbListApi.md#add_db_list_item) | **POST** /dblist/{listName}/items | Add an item to a DBList
*DbListApi* | [**check_db_list_item_exists**](docs/DbListApi.md#check_db_list_item_exists) | **POST** /dblist/{listName}/exists | Check if an item exists in a DBList
*DbListApi* | [**delete_db_list_item**](docs/DbListApi.md#delete_db_list_item) | **DELETE** /dblist/items/{itemId} | Delete an item from a DBList
*DbListApi* | [**list_db_list_items**](docs/DbListApi.md#list_db_list_items) | **GET** /dblist/{listName}/items | List items from a specific DBList
*DbListApi* | [**list_db_lists**](docs/DbListApi.md#list_db_lists) | **GET** /dblist | List all DBList names
*DbListApi* | [**update_db_list_item**](docs/DbListApi.md#update_db_list_item) | **PUT** /dblist/items/{itemId} | Update an item in a DBList
*JobApi* | [**create_analyzer_job**](docs/JobApi.md#create_analyzer_job) | **POST** /analyzer/{workerId}/run | Create and run an analyzer job
*JobApi* | [**create_responder_job**](docs/JobApi.md#create_responder_job) | **POST** /responder/{workerId}/run | Create and run a responder job
*JobApi* | [**delete_job**](docs/JobApi.md#delete_job) | **DELETE** /job/{jobId} | Delete a job
*JobApi* | [**find_jobs**](docs/JobApi.md#find_jobs) | **POST** /job/_search | Find/Search jobs
*JobApi* | [**get_job_by_id**](docs/JobApi.md#get_job_by_id) | **GET** /job/{jobId} | Get a specific job by ID
*JobApi* | [**get_job_report**](docs/JobApi.md#get_job_report) | **GET** /job/{jobId}/report | Get the report for a job
*JobApi* | [**get_jobs_status**](docs/JobApi.md#get_jobs_status) | **POST** /job/status | Get the status of multiple jobs
*JobApi* | [**list_job_artifacts**](docs/JobApi.md#list_job_artifacts) | **POST** /job/{jobId}/artifacts/_search | List/Search artifacts for a job
*JobApi* | [**list_jobs**](docs/JobApi.md#list_jobs) | **GET** /job | List jobs for the user with optional filters
*JobApi* | [**wait_job_report**](docs/JobApi.md#wait_job_report) | **GET** /job/{jobId}/waitReport | Wait for and get the report for a job
*MispApi* | [**list_misp_modules**](docs/MispApi.md#list_misp_modules) | **POST** /misp/modules | List available MISP modules (Cortex analyzers)
*MispApi* | [**query_misp_module**](docs/MispApi.md#query_misp_module) | **POST** /misp/query | Query a MISP module (Cortex analyzer)
*OrganizationApi* | [**create_organization**](docs/OrganizationApi.md#create_organization) | **POST** /organization | Create a new organization
*OrganizationApi* | [**delete_organization**](docs/OrganizationApi.md#delete_organization) | **DELETE** /organization/{organizationId} | Delete an organization
*OrganizationApi* | [**find_organizations**](docs/OrganizationApi.md#find_organizations) | **POST** /organization/_search | Find/Search organizations
*OrganizationApi* | [**get_organization_by_id**](docs/OrganizationApi.md#get_organization_by_id) | **GET** /organization/{organizationId} | Get a specific organization by ID
*OrganizationApi* | [**get_organization_stats**](docs/OrganizationApi.md#get_organization_stats) | **POST** /organization/_stats | Get statistics for organizations
*OrganizationApi* | [**update_organization**](docs/OrganizationApi.md#update_organization) | **PUT** /organization/{organizationId} | Update an organization
*ResponderApi* | [**create_responder_from_definition**](docs/ResponderApi.md#create_responder_from_definition) | **POST** /responder/definition/{responderDefinitionId} | Create a new responder instance from a definition
*ResponderApi* | [**delete_responder**](docs/ResponderApi.md#delete_responder) | **DELETE** /responder/{responderId} | Delete a responder
*ResponderApi* | [**find_responders**](docs/ResponderApi.md#find_responders) | **POST** /responder/_search | Find/Search responders
*ResponderApi* | [**get_responder_by_id**](docs/ResponderApi.md#get_responder_by_id) | **GET** /responder/{responderId} | Get a specific responder by ID
*ResponderApi* | [**list_responder_definitions**](docs/ResponderApi.md#list_responder_definitions) | **GET** /responder/definition | List all available responder definitions
*ResponderApi* | [**list_responders_for_type**](docs/ResponderApi.md#list_responders_for_type) | **GET** /responder/type/{dataType} | List responders available for a specific data type
*ResponderApi* | [**scan_responder_definitions**](docs/ResponderApi.md#scan_responder_definitions) | **POST** /responder/scan | Trigger a rescan of responder definitions
*ResponderApi* | [**update_responder**](docs/ResponderApi.md#update_responder) | **PUT** /responder/{responderId} | Update a responder
*ResponderConfigApi* | [**get_responder_configuration**](docs/ResponderConfigApi.md#get_responder_configuration) | **GET** /responder/config/{responderConfigName} | Get a specific responder configuration
*ResponderConfigApi* | [**list_responder_configurations**](docs/ResponderConfigApi.md#list_responder_configurations) | **GET** /responder/config | List all responder configurations for the user
*ResponderConfigApi* | [**update_responder_configuration**](docs/ResponderConfigApi.md#update_responder_configuration) | **PUT** /responder/config/{responderConfigName} | Update or create a responder configuration
*StatusApi* | [**get_health_status**](docs/StatusApi.md#get_health_status) | **GET** /status/health | Get system health status
*StatusApi* | [**get_status**](docs/StatusApi.md#get_status) | **GET** /status | Get system status and configuration information
*StatusApi* | [**get_status_alerts**](docs/StatusApi.md#get_status_alerts) | **GET** /status/alerts | Get system alerts
*StreamApi* | [**create_stream**](docs/StreamApi.md#create_stream) | **POST** /stream | Create a new event stream
*StreamApi* | [**get_stream_events**](docs/StreamApi.md#get_stream_events) | **GET** /stream/{id} | Get events from a stream
*StreamApi* | [**get_stream_session_status**](docs/StreamApi.md#get_stream_session_status) | **GET** /stream/status | Get the status of the current session/token for streaming
*UserApi* | [**change_user_password**](docs/UserApi.md#change_user_password) | **POST** /user/{userId}/password/change | Change the current user's password
*UserApi* | [**create_user**](docs/UserApi.md#create_user) | **POST** /user | Create a new user
*UserApi* | [**delete_user**](docs/UserApi.md#delete_user) | **DELETE** /user/{userId} | Delete a user (marks as Locked)
*UserApi* | [**find_users**](docs/UserApi.md#find_users) | **POST** /user/_search | Find/Search users (SuperAdmin only)
*UserApi* | [**find_users_for_organization**](docs/UserApi.md#find_users_for_organization) | **POST** /user/organization/{organizationId}/_search | Find/Search users within a specific organization
*UserApi* | [**get_current_user**](docs/UserApi.md#get_current_user) | **GET** /user/current | Get the current logged-in user's details
*UserApi* | [**get_user_api_key**](docs/UserApi.md#get_user_api_key) | **GET** /user/{userId}/key | Get a user's API key
*UserApi* | [**get_user_by_id**](docs/UserApi.md#get_user_by_id) | **GET** /user/{userId} | Get a specific user by ID
*UserApi* | [**remove_user_api_key**](docs/UserApi.md#remove_user_api_key) | **DELETE** /user/{userId}/key | Remove a user's API key
*UserApi* | [**renew_user_api_key**](docs/UserApi.md#renew_user_api_key) | **POST** /user/{userId}/key/renew | Renew a user's API key
*UserApi* | [**set_user_password**](docs/UserApi.md#set_user_password) | **POST** /user/{userId}/password/set | Set a user's password (admin)
*UserApi* | [**update_user**](docs/UserApi.md#update_user) | **PUT** /user/{userId} | Update a user


## Documentation For Models

 - [AnalyzerConfigUpdateRequest](docs/AnalyzerConfigUpdateRequest.md)
 - [AnalyzerCreateRequest](docs/AnalyzerCreateRequest.md)
 - [AnalyzerFindRequest](docs/AnalyzerFindRequest.md)
 - [Artifact](docs/Artifact.md)
 - [Attachment](docs/Attachment.md)
 - [AuthContext](docs/AuthContext.md)
 - [BaseConfig](docs/BaseConfig.md)
 - [ConfigurationDefinitionItem](docs/ConfigurationDefinitionItem.md)
 - [ConfigurationDefinitionItemDefaultValue](docs/ConfigurationDefinitionItemDefaultValue.md)
 - [DbListAddItemRequest](docs/DbListAddItemRequest.md)
 - [DbListItemExistsRequest](docs/DbListItemExistsRequest.md)
 - [DbListItemExistsResponse](docs/DbListItemExistsResponse.md)
 - [Error](docs/Error.md)
 - [FindAnalyzers200Response](docs/FindAnalyzers200Response.md)
 - [FindOrganizations200Response](docs/FindOrganizations200Response.md)
 - [FindUsers200Response](docs/FindUsers200Response.md)
 - [HealthResponse](docs/HealthResponse.md)
 - [Job](docs/Job.md)
 - [JobCreateRequest](docs/JobCreateRequest.md)
 - [JobData](docs/JobData.md)
 - [JobReportResponse](docs/JobReportResponse.md)
 - [JobReportResponseOneOf](docs/JobReportResponseOneOf.md)
 - [JobStatusRequest](docs/JobStatusRequest.md)
 - [ListAnalyzerDefinitions200Response](docs/ListAnalyzerDefinitions200Response.md)
 - [ListJobArtifacts200Response](docs/ListJobArtifacts200Response.md)
 - [ListJobs200Response](docs/ListJobs200Response.md)
 - [ListMispModules200Response](docs/ListMispModules200Response.md)
 - [LoginRequest](docs/LoginRequest.md)
 - [MispModule](docs/MispModule.md)
 - [MispModuleMeta](docs/MispModuleMeta.md)
 - [MispModuleMispattributes](docs/MispModuleMispattributes.md)
 - [MispQueryRequest](docs/MispQueryRequest.md)
 - [MispQueryResponse](docs/MispQueryResponse.md)
 - [MispQueryResponseResultsInner](docs/MispQueryResponseResultsInner.md)
 - [Organization](docs/Organization.md)
 - [OrganizationCreateRequest](docs/OrganizationCreateRequest.md)
 - [OrganizationFindRequest](docs/OrganizationFindRequest.md)
 - [OrganizationStatsRequest](docs/OrganizationStatsRequest.md)
 - [OrganizationUpdateRequest](docs/OrganizationUpdateRequest.md)
 - [PasswordChangeRequest](docs/PasswordChangeRequest.md)
 - [PasswordSetRequest](docs/PasswordSetRequest.md)
 - [ResponderConfigUpdateRequest](docs/ResponderConfigUpdateRequest.md)
 - [ResponderCreateRequest](docs/ResponderCreateRequest.md)
 - [ResponderFindRequest](docs/ResponderFindRequest.md)
 - [StatusResponse](docs/StatusResponse.md)
 - [StatusResponseConfig](docs/StatusResponseConfig.md)
 - [StatusResponseConfigAuthType](docs/StatusResponseConfigAuthType.md)
 - [StatusResponseVersions](docs/StatusResponseVersions.md)
 - [StreamMessagesResponse](docs/StreamMessagesResponse.md)
 - [StreamStatusResponse](docs/StreamStatusResponse.md)
 - [User](docs/User.md)
 - [UserCreateRequest](docs/UserCreateRequest.md)
 - [UserUpdateRequest](docs/UserUpdateRequest.md)
 - [Worker](docs/Worker.md)
 - [WorkerConfiguration](docs/WorkerConfiguration.md)
 - [WorkerDefinition](docs/WorkerDefinition.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```
