/*
 * IAM Identity Services API
 *
 * The IAM Identity Service API allows for the management of Account Settings and Identities (Service IDs, ApiKeys).
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountSettingsResponse : Response body format for Account Settings REST requests.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSettingsResponse {
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<crate::models::ResponseContext>>,
    /// Unique ID of the account.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Defines whether or not creating a Service Id is access controlled. Valid values:   * RESTRICTED - to apply access control   * NOT_RESTRICTED - to remove access control   * NOT_SET - to 'unset' a previous set value
    #[serde(rename = "restrict_create_service_id")]
    pub restrict_create_service_id: RestrictCreateServiceId,
    /// Defines whether or not creating platform API keys is access controlled. Valid values:   * RESTRICTED - to apply access control   * NOT_RESTRICTED - to remove access control   * NOT_SET - to 'unset' a previous set value
    #[serde(rename = "restrict_create_platform_apikey")]
    pub restrict_create_platform_apikey: RestrictCreatePlatformApikey,
    /// Defines the IP addresses and subnets from which IAM tokens can be created for the account.
    #[serde(rename = "allowed_ip_addresses")]
    pub allowed_ip_addresses: String,
    /// Version of the account settings.
    #[serde(rename = "entity_tag")]
    pub entity_tag: String,
    /// Defines the MFA trait for the account. Valid values:   * NONE - No MFA trait set   * TOTP - For all non-federated IBMId users   * TOTP4ALL - For all users   * LEVEL1 - Email-based MFA for all users   * LEVEL2 - TOTP-based MFA for all users   * LEVEL3 - U2F MFA for all users 
    #[serde(rename = "mfa")]
    pub mfa: Mfa,
    /// History of the Account Settings.
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<crate::models::EnityHistoryRecord>>,
    /// Defines the session expiration in seconds for the account. Valid values:   * Any whole number between between '900' and '86400'   * NOT_SET - To unset account setting and use service default
    #[serde(rename = "session_expiration_in_seconds")]
    pub session_expiration_in_seconds: String,
    /// Defines the period of time in seconds in which a session will be invalidated due  to inactivity. Valid values:    * Any whole number between '900' and '7200'    * NOT_SET - To unset account setting and use service default
    #[serde(rename = "session_invalidation_in_seconds")]
    pub session_invalidation_in_seconds: String,
    /// Defines the max allowed sessions per identity required by the account. Valid values:   * Any whole number greater than 0   * NOT_SET - To unset account setting and use service default
    #[serde(rename = "max_sessions_per_identity")]
    pub max_sessions_per_identity: String,
}

impl AccountSettingsResponse {
    /// Response body format for Account Settings REST requests.
    pub fn new(account_id: String, restrict_create_service_id: RestrictCreateServiceId, restrict_create_platform_apikey: RestrictCreatePlatformApikey, allowed_ip_addresses: String, entity_tag: String, mfa: Mfa, session_expiration_in_seconds: String, session_invalidation_in_seconds: String, max_sessions_per_identity: String) -> AccountSettingsResponse {
        AccountSettingsResponse {
            context: None,
            account_id,
            restrict_create_service_id,
            restrict_create_platform_apikey,
            allowed_ip_addresses,
            entity_tag,
            mfa,
            history: None,
            session_expiration_in_seconds,
            session_invalidation_in_seconds,
            max_sessions_per_identity,
        }
    }
}

/// Defines whether or not creating a Service Id is access controlled. Valid values:   * RESTRICTED - to apply access control   * NOT_RESTRICTED - to remove access control   * NOT_SET - to 'unset' a previous set value
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RestrictCreateServiceId {
    #[serde(rename = "RESTRICTED")]
    RESTRICTED,
    #[serde(rename = "NOT_RESTRICTED")]
    NOTRESTRICTED,
    #[serde(rename = "NOT_SET")]
    NOTSET,
}
/// Defines whether or not creating platform API keys is access controlled. Valid values:   * RESTRICTED - to apply access control   * NOT_RESTRICTED - to remove access control   * NOT_SET - to 'unset' a previous set value
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RestrictCreatePlatformApikey {
    #[serde(rename = "RESTRICTED")]
    RESTRICTED,
    #[serde(rename = "NOT_RESTRICTED")]
    NOTRESTRICTED,
    #[serde(rename = "NOT_SET")]
    NOTSET,
}
/// Defines the MFA trait for the account. Valid values:   * NONE - No MFA trait set   * TOTP - For all non-federated IBMId users   * TOTP4ALL - For all users   * LEVEL1 - Email-based MFA for all users   * LEVEL2 - TOTP-based MFA for all users   * LEVEL3 - U2F MFA for all users 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mfa {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "TOTP")]
    TOTP,
    #[serde(rename = "TOTP4ALL")]
    TOTP4ALL,
    #[serde(rename = "LEVEL1")]
    LEVEL1,
    #[serde(rename = "LEVEL2")]
    LEVEL2,
    #[serde(rename = "LEVEL3")]
    LEVEL3,
}

