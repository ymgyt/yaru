use http::Uri;
use serde::{Deserialize, Serialize};

pub mod github;

/// https://datatracker.ietf.org/doc/html/rfc8628#section-3.1
#[derive(Serialize)]
struct DeviceAuthorizationRequest<'s> {
    client_id: &'s str,
    scope: &'s str,
}

/// https://datatracker.ietf.org/doc/html/rfc8628#section-3.2
#[derive(Debug, Deserialize)]
struct DeviceAuthorizationResponse {
    /// device verification code
    device_code: String,
    /// end user verification code
    user_code: String,
    /// end user verification uri on the authorization server
    #[serde(with = "http_serde_ext::uri")]
    verification_uri: Uri,
    /// a verification uri that includes user_code which is designed for non-textual transmission.
    // error if there is no field on deserializing, maybe bug on http_serde_ext crate ?
    #[allow(unused)]
    #[serde(with = "http_serde_ext::uri::option", skip_deserializing)]
    verification_uri_complete: Option<Uri>,
    /// the lifetime in seconds of the device_code and user_code
    #[allow(unused)]
    expires_in: i64,
    /// the minimum amount of time in seconds that the client should wait between polling requests to the token endpoint
    /// if no value is provided, clients must use 5 as the default
    interval: Option<i64>,
}

#[derive(Serialize)]
struct DeviceAccessTokenRequest<'s> {
    /// Value MUST be set to "urn:ietf:params:oauth:grant-type:device_code"
    grant_type: &'static str,
    /// The device verification code, "device_code" from the device authorization response
    device_code: &'s str,
    client_id: &'static str,
}

impl<'s> DeviceAccessTokenRequest<'s> {
    const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";

    fn new(device_code: &'s str, client_id: &'static str) -> Self {
        Self {
            grant_type: Self::GRANT_TYPE,
            device_code,
            client_id,
        }
    }
}

/// Successful Response
/// https://datatracker.ietf.org/doc/html/rfc6749#section-5.1
#[derive(Deserialize, Debug)]
pub struct DeviceAccessTokenResponse {
    /// the access token issued by the authorization server
    pub access_token: String,
    pub token_type: String,
    /// the lifetime in seconds of the access token
    pub expires_in: Option<i64>,
}

/// https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
#[derive(Deserialize, Debug)]
struct DeviceAccessTokenErrorResponse {
    error: DeviceAccessTokenErrorCode,
    #[allow(unused)]
    error_description: Option<String>,
    // error if there is no field on deserializing, maybe bug on http_serde_ext crate ?
    #[allow(unused)]
    #[serde(with = "http_serde_ext::uri::option", skip_deserializing)]
    error_uri: Option<Uri>,
}

#[derive(PartialEq, Eq, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DeviceAccessTokenErrorCode {
    AuthorizationPending,
    SlowDown,
    AccessDenied,
    ExpiredToken,
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
}

impl DeviceAccessTokenErrorCode {
    ///  The "authorization_pending" and "slow_down" error codes define particularly unique behavior, as they indicate that the OAuth client should continue to poll the token endpoint by repeating the token request (implementing the precise behavior defined above)
    fn should_continue_to_poll(&self) -> bool {
        use DeviceAccessTokenErrorCode::*;
        *self == AuthorizationPending || *self == SlowDown
    }
}
