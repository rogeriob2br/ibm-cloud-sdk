use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use anyhow::Result;
use serde::Deserialize;

const GRANT_TYPE: &str = "urn:ibm:params:oauth:grant-type:apikey";

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AuthenticatorApiClient {
    url: String,
}

impl AuthenticatorApiClient {
    pub fn new(url: String) -> AuthenticatorApiClient {
        AuthenticatorApiClient { url }
    }
    pub async fn authenticate(&self, req: TokenApiKeyRequest) -> Result<ResponseType> {

        let response = get_token(req, String::from(&self.url)).await?;
        match response.clone(){
            TokenResponse=>{
                Ok(TokenResponse)
            }

        }
    }
}

async fn get_token(req: TokenApiKeyRequest, url: String) -> Result<ResponseType> {
    let params = urlencoded_parameter(req);
    let response: ResponseType = reqwest::Client::new()
        .post(url)
        .form(&params)
        .headers(construct_headers())
        .send()
        .await?.
        json().
        await?;
    Ok(response)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TokenApiKeyRequest {
    //Grant type for this API call. You must set the
    // grant type to urn:ibm:params:oauth:grant-type:apikey.
    grant_type: String,

    //The value of the api key.
    apikey: String,
}

impl TokenApiKeyRequest {
    pub fn new(apikey: String) -> TokenApiKeyRequest {
        TokenApiKeyRequest {
            grant_type: GRANT_TYPE.to_string(),
            apikey,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseType{
    Ok(TokenResponse),
    Err(OidcExceptionResponse)
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenResponse {
    //Response body for POST /identity/token.

    //The IAM access token that can be used to invoke various IBM Cloud APIs.
    // Use this token with the prefix Bearer in the HTTP header Authorization
    // for invocations of IAM compatible APIs.
    access_token: String,

    //(optional) A refresh token that can be used to get a new IAM access
    // token if that token is expired. When using the default client
    // (no basic authorization header) as described in this documentation,
    // this refresh_token cannot be used to retrieve a new IAM access token.
    // When the IAM access token is about to be expired, use the API key to
    // create a new access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,

    //(optional) A delegated refresh token that can only be consumed by
    // the clients that have been specified in the API call as 'receiver_client_ids
    #[serde(skip_serializing_if = "Option::is_none")]
    delegated_refresh_token: Option<String>,

    //The type of the token. Currently, only Bearer is returned.
    token_type: String,

    // Number of seconds until the IAM access token will expire.
    expires_in: i32,

    // Number of seconds counted since January 1st, 1970, until the IAM access token will expire.
    expiration: i32,
}

impl TokenResponse{
    pub fn get_access_token(&self)->String{
        self.access_token.clone()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExceptionResponseContext {
    //Context fill with key properties for problem determination.

    //The request ID of the inbound REST request.
    #[serde(rename = "requestId")]
    request_id: String,

    //The request type of the inbound REST request.
    #[serde(rename = "requestType")]
    request_type: String,

    //The user agent of the inbound REST request.
    #[serde(rename = "userAgent")]
    user_agent: String,

    // The URL of that cluster.
    url: String,

    //The instance ID of the server instance processing the request.
    #[serde(rename = "instanceId")]
    instance_id: String,

    //The thread ID of the server instance processing the request.
    #[serde(rename = "threadId")]
    thread_id: String,

    //The host of the server instance processing the request.
    host: String,

    //The start time of the request.
    #[serde(rename = "startTime")]
    start_time: String,

    //The finish time of the request.
    #[serde(rename = "endTime")]
    end_time: String,

    //The elapsed time in msec.
    #[serde(rename = "elapsedTime")]
    elapsed_time: String,

    //The language used to present the error message.
    locale: String,

    //The cluster name.
    #[serde(rename = "clusterName")]
    cluster_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MFARequirementsResponse {
    //Response properties for MFA requirements.

    //MFA error.
    error: String,

    //MFA Code.
    code: String,

    //MFA AuthorizationToken.
    authorizationToken: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OidcExceptionResponse {
    // Response body parameters in case of oidc error situations.

    // Context fill with key properties for problem determination.

    context: ExceptionResponseContext,

    //Error message code of the REST Exception.
    #[serde(rename = "errorCode")]
    error_code: String,

    //Error message of the REST Exception. Error messages are derived base on the input locale
    // of the REST request and the available Message catalogs. Dynamic fallback to 'us-english'
    // is happening if no message catalog is available for the provided input locale.
    #[serde(rename = "errorMessage")]
    error_message: String,

    //Error details of the REST Exception.

    //#[serde(rename = "errorDetails")]
    //error_details: String,

    //Response properties for MFA requirements.
    //#[serde(skip_deserializing)]
    //requirements: MFARequirementsResponse,
}

/// Helpers------------------------------------------------

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers
}


fn urlencoded_parameter(token: TokenApiKeyRequest) -> [(String, String); 2] {
    let params: [(String, String); 2] = [
        ("grant_type".to_string(), token.grant_type),
        ("apikey".to_string(), token.apikey),
    ];
    params
}


#[cfg(test)]
mod TokenApiTests{
    const ibm_cloud_iam_url: &str = "ibm_cloud_iam_url";
    const api_key:  &str= "api_key";
    const GRANT_TYPE: &str = "urn:ibm:params:oauth:grant-type:apikey";

    use crate::authenticators::token_api::{AuthenticatorApiClient, TokenApiKeyRequest};

    #[test]
    fn new_authenticator_client_success(){
        let auth = AuthenticatorApiClient::new(ibm_cloud_iam_url.to_string());

        assert_eq!(auth, AuthenticatorApiClient{ url: ibm_cloud_iam_url.to_string() })
    }
    fn new_token_api_request_success(){
        let req = TokenApiKeyRequest::new(api_key.to_string());

        assert_eq!(req, TokenApiKeyRequest{ grant_type: GRANT_TYPE.to_string(), apikey: api_key.to_string() })
    }



}

