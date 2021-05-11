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
pub struct DialogSuggestion {
    /// The user-facing label for the suggestion. This label is taken from the **title** or **user_label** property of the corresponding dialog node, depending on the disambiguation options.
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "value")]
    pub value: Box<crate::models::DialogSuggestionValue>,
    /// The dialog output that will be returned from the Watson Assistant service if the user selects the corresponding option.
    #[serde(rename = "output", skip_serializing_if = "Option::is_none")]
    pub output: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl DialogSuggestion {
    pub fn new(label: String, value: crate::models::DialogSuggestionValue) -> DialogSuggestion {
        DialogSuggestion {
            label,
            value: Box::new(value),
            output: None,
        }
    }
}


