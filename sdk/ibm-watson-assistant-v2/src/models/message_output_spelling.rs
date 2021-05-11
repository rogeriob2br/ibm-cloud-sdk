/*
 * Watson Assistant v2
 *
 * The IBM Watson&trade; Assistant service combines machine learning, natural language understanding, and an integrated dialog editor to create conversation flows between your apps and your users.  The Assistant v2 API provides runtime methods your client application can use to send user input to an assistant and receive a response.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MessageOutputSpelling : Properties describing any spelling corrections in the user input that was received.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageOutputSpelling {
    /// The user input text that was used to generate the response. If spelling autocorrection is enabled, this text reflects any spelling corrections that were applied.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The original user input text. This property is returned only if autocorrection is enabled and the user input was corrected.
    #[serde(rename = "original_text", skip_serializing_if = "Option::is_none")]
    pub original_text: Option<String>,
    /// Any suggested corrections of the input text. This property is returned only if spelling correction is enabled and autocorrection is disabled.
    #[serde(rename = "suggested_text", skip_serializing_if = "Option::is_none")]
    pub suggested_text: Option<String>,
}

impl MessageOutputSpelling {
    /// Properties describing any spelling corrections in the user input that was received.
    pub fn new() -> MessageOutputSpelling {
        MessageOutputSpelling {
            text: None,
            original_text: None,
            suggested_text: None,
        }
    }
}


