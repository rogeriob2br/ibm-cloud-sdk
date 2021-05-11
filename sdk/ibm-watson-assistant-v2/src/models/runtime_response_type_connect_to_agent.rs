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
pub struct RuntimeResponseTypeConnectToAgent {
    /// The type of response returned by the dialog node. The specified response type must be supported by the client application or channel.
    #[serde(rename = "response_type")]
    pub response_type: String,
    /// A message to be sent to the human agent who will be taking over the conversation.
    #[serde(rename = "message_to_human_agent", skip_serializing_if = "Option::is_none")]
    pub message_to_human_agent: Option<String>,
    #[serde(rename = "agent_available", skip_serializing_if = "Option::is_none")]
    pub agent_available: Option<Box<crate::models::AgentAvailabilityMessage>>,
    #[serde(rename = "agent_unavailable", skip_serializing_if = "Option::is_none")]
    pub agent_unavailable: Option<Box<crate::models::AgentAvailabilityMessage>>,
    #[serde(rename = "transfer_info", skip_serializing_if = "Option::is_none")]
    pub transfer_info: Option<Box<crate::models::DialogNodeOutputConnectToAgentTransferInfo>>,
    /// A label identifying the topic of the conversation, derived from the **title** property of the relevant node or the **topic** property of the dialog node response.
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// An array of objects specifying channels for which the response is intended. If **channels** is present, the response is intended for a built-in integration and should not be handled by an API client.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<crate::models::ResponseGenericChannel>>,
}

impl RuntimeResponseTypeConnectToAgent {
    pub fn new(response_type: String) -> RuntimeResponseTypeConnectToAgent {
        RuntimeResponseTypeConnectToAgent {
            response_type,
            message_to_human_agent: None,
            agent_available: None,
            agent_unavailable: None,
            transfer_info: None,
            topic: None,
            channels: None,
        }
    }
}


