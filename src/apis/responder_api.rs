/*
 * Cortex API
 *
 * API for Cortex, a powerful observable analysis and active response engine.
 *
 * The version of the OpenAPI document: 3.1.8
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_responder_from_definition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateResponderFromDefinitionError {
    Status400(models::Error),
    Status401(models::Error),
    Status403(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_responder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteResponderError {
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`find_responders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindRespondersError {
    Status401(models::Error),
    Status403(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_responder_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetResponderByIdError {
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_responder_definitions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListResponderDefinitionsError {
    Status401(models::Error),
    Status403(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_responders_for_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRespondersForTypeError {
    Status401(models::Error),
    Status403(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`scan_responder_definitions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScanResponderDefinitionsError {
    Status401(models::Error),
    Status403(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_responder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateResponderError {
    Status400(models::Error),
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}


pub async fn create_responder_from_definition(configuration: &configuration::Configuration, responder_definition_id: &str, responder_create_request: models::ResponderCreateRequest) -> Result<models::Worker, Error<CreateResponderFromDefinitionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_responder_definition_id = responder_definition_id;
    let p_responder_create_request = responder_create_request;

    let uri_str = format!("{}/responder/definition/{responderDefinitionId}", configuration.base_path, responderDefinitionId=crate::apis::urlencode(p_responder_definition_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_responder_create_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Worker`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Worker`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateResponderFromDefinitionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_responder(configuration: &configuration::Configuration, responder_id: &str) -> Result<(), Error<DeleteResponderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_responder_id = responder_id;

    let uri_str = format!("{}/responder/{responderId}", configuration.base_path, responderId=crate::apis::urlencode(p_responder_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteResponderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn find_responders(configuration: &configuration::Configuration, responder_find_request: Option<models::ResponderFindRequest>) -> Result<models::ListAnalyzersForType200Response, Error<FindRespondersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_responder_find_request = responder_find_request;

    let uri_str = format!("{}/responder/_search", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_responder_find_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListAnalyzersForType200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListAnalyzersForType200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<FindRespondersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_responder_by_id(configuration: &configuration::Configuration, responder_id: &str) -> Result<models::Worker, Error<GetResponderByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_responder_id = responder_id;

    let uri_str = format!("{}/responder/{responderId}", configuration.base_path, responderId=crate::apis::urlencode(p_responder_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Worker`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Worker`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetResponderByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_responder_definitions(configuration: &configuration::Configuration, ) -> Result<models::ListAnalyzerDefinitions200Response, Error<ListResponderDefinitionsError>> {

    let uri_str = format!("{}/responder/definition", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListAnalyzerDefinitions200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListAnalyzerDefinitions200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListResponderDefinitionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_responders_for_type(configuration: &configuration::Configuration, data_type: &str) -> Result<models::ListAnalyzersForType200Response, Error<ListRespondersForTypeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_data_type = data_type;

    let uri_str = format!("{}/responder/type/{dataType}", configuration.base_path, dataType=crate::apis::urlencode(p_data_type));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListAnalyzersForType200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListAnalyzersForType200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListRespondersForTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn scan_responder_definitions(configuration: &configuration::Configuration, ) -> Result<(), Error<ScanResponderDefinitionsError>> {

    let uri_str = format!("{}/responder/scan", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ScanResponderDefinitionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_responder(configuration: &configuration::Configuration, responder_id: &str, request_body: std::collections::HashMap<String, serde_json::Value>) -> Result<models::Worker, Error<UpdateResponderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_responder_id = responder_id;
    let p_request_body = request_body;

    let uri_str = format!("{}/responder/{responderId}", configuration.base_path, responderId=crate::apis::urlencode(p_responder_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_request_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Worker`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Worker`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateResponderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

