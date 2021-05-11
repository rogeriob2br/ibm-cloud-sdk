/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_token_api_key`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTokenApiKeyError {
    Status400(crate::models::OidcExceptionResponse),
    Status401(crate::models::OidcExceptionResponse),
    Status403(crate::models::OidcExceptionResponse),
    Status500(crate::models::OidcExceptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_token_api_key_delegated_refresh_token`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTokenApiKeyDelegatedRefreshTokenError {
    Status400(crate::models::OidcExceptionResponse),
    Status401(crate::models::OidcExceptionResponse),
    Status403(crate::models::OidcExceptionResponse),
    Status500(crate::models::OidcExceptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_token_iam_authz`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTokenIamAuthzError {
    Status400(crate::models::OidcExceptionResponse),
    Status401(crate::models::OidcExceptionResponse),
    Status403(crate::models::OidcExceptionResponse),
    Status500(crate::models::OidcExceptionResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_token_password`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTokenPasswordError {
    Status400(crate::models::OidcExceptionResponse),
    Status401(crate::models::OidcExceptionResponse),
    Status403(crate::models::OidcExceptionResponse),
    Status500(crate::models::OidcExceptionResponse),
    UnknownValue(serde_json::Value),
}


/// Creates a non-opaque access token for an API key.
pub async fn get_token_api_key(configuration: &configuration::Configuration, grant_type: &str, apikey: &str) -> Result<crate::models::TokenResponse, Error<GetTokenApiKeyError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/identity/token#apikey", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("grant_type", grant_type.to_string());
    local_var_form_params.insert("apikey", apikey.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTokenApiKeyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a non-opaque access token and a delegated refresh token for an API key.
pub async fn get_token_api_key_delegated_refresh_token(configuration: &configuration::Configuration, grant_type: &str, apikey: &str, response_type: &str, receiver_client_ids: &str, delegated_refresh_token_expiry: Option<i32>) -> Result<crate::models::TokenResponse, Error<GetTokenApiKeyDelegatedRefreshTokenError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/identity/token#apikey-delegated-refresh-token", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("grant_type", grant_type.to_string());
    local_var_form_params.insert("apikey", apikey.to_string());
    local_var_form_params.insert("response_type", response_type.to_string());
    local_var_form_params.insert("receiver_client_ids", receiver_client_ids.to_string());
    if let Some(local_var_param_value) = delegated_refresh_token_expiry {
        local_var_form_params.insert("delegated_refresh_token_expiry", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTokenApiKeyDelegatedRefreshTokenError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a non-opaque access token, if an appropriate authorization policy is in place. This kind of IAM access token is typically used for access between services.
pub async fn get_token_iam_authz(configuration: &configuration::Configuration, grant_type: &str, access_token: &str, desired_iam_id: &str) -> Result<crate::models::TokenResponse, Error<GetTokenIamAuthzError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/identity/token#iam-authz", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("grant_type", grant_type.to_string());
    local_var_form_params.insert("access_token", access_token.to_string());
    local_var_form_params.insert("desired_iam_id", desired_iam_id.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTokenIamAuthzError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a non-opaque access token for a username and password. To be able to call IBM Cloud APIs, the token must be made account-specific. For this purpose, also pass the 32 character long identifier for your account in the API call. This API call is possible only for non-federated IBMid users.
pub async fn get_token_password(configuration: &configuration::Configuration, authorization: &str, grant_type: &str, username: &str, password: &str, account: Option<&str>) -> Result<crate::models::TokenResponse, Error<GetTokenPasswordError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/identity/token#password", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Authorization", authorization.to_string());
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("grant_type", grant_type.to_string());
    local_var_form_params.insert("username", username.to_string());
    local_var_form_params.insert("password", password.to_string());
    if let Some(local_var_param_value) = account {
        local_var_form_params.insert("account", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTokenPasswordError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

