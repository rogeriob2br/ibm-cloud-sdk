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
pub struct BulkClassifyOutput {
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<Box<crate::models::BulkClassifyUtterance>>,
    /// An array of entities identified in the utterance.
    #[serde(rename = "entities", skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::models::RuntimeEntity>>,
    /// An array of intents recognized in the utterance.
    #[serde(rename = "intents", skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<crate::models::RuntimeIntent>>,
}

impl BulkClassifyOutput {
    pub fn new() -> BulkClassifyOutput {
        BulkClassifyOutput {
            input: None,
            entities: None,
            intents: None,
        }
    }
}


