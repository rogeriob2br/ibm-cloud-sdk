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
pub struct RuntimeEntityInterpretationSysNumber {
    /// A recognized numeric value, represented as an integer or double.
    #[serde(rename = "numeric_value", skip_serializing_if = "Option::is_none")]
    pub numeric_value: Option<f32>,
    /// A unique identifier used to associate multiple recognized `@sys-date`, `@sys-time`, or `@sys-number` entities that are recognized as a range of values in the user's input (for example, `from July 4 until July 14` or `from 20 to 25`).
    #[serde(rename = "range_link", skip_serializing_if = "Option::is_none")]
    pub range_link: Option<String>,
    /// The type of numeric value recognized in the user input (`integer` or `rational`).
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
}

impl RuntimeEntityInterpretationSysNumber {
    pub fn new() -> RuntimeEntityInterpretationSysNumber {
        RuntimeEntityInterpretationSysNumber {
            numeric_value: None,
            range_link: None,
            subtype: None,
        }
    }
}


