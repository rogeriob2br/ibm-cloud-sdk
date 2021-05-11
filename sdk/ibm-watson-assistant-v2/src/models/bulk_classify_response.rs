/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkClassifyResponse {
    /// An array of objects that contain classification information for the submitted input utterances.
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<crate::models::BulkClassifyOutput>>,
}

impl BulkClassifyResponse {
    pub fn new() -> BulkClassifyResponse {
        BulkClassifyResponse {
            output: None,
        }
    }
}


