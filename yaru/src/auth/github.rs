use std::time::Duration;

use http::StatusCode;
use reqwest::Client;
use tracing::debug;

use crate::{
    auth::{DeviceAccessTokenErrorResponse, DeviceAccessTokenRequest},
    config,
};

use super::{DeviceAccessTokenResponse, DeviceAuthorizationRequest, DeviceAuthorizationResponse};

/// https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow
pub struct DeviceFlow {
    client: Client,
    client_id: &'static str,
}

impl DeviceFlow {
    const DEVICE_AUTHORIZATION_ENDPOINT: &str = "https://github.com/login/device/code";
    const TOKEN_ENDPOINT: &str = "https://github.com/login/oauth/access_token";

    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .user_agent(config::USER_AGENT)
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        Self {
            client,
            client_id: config::github::CLIENT_ID,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn device_flow(self) -> anyhow::Result<DeviceAccessTokenResponse> {
        // https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps
        let scope = "user:email";

        let response = self
            .client
            .post(Self::DEVICE_AUTHORIZATION_ENDPOINT)
            .header(http::header::ACCEPT, "application/json")
            .form(&DeviceAuthorizationRequest {
                client_id: self.client_id,
                scope,
            })
            .send()
            .await?
            .error_for_status()?
            .json::<DeviceAuthorizationResponse>()
            .await?;

        debug!("{response:?}");

        let DeviceAuthorizationResponse {
            device_code,
            user_code,
            verification_uri,
            interval,
            ..
        } = response;

        println!("Open `{verification_uri}` on your browser");
        println!("Enter CODE: `{user_code}`");

        // attempt to open input screen in the browser
        open::that(verification_uri.to_string()).ok();

        // poll to check if user authorized the device
        macro_rules! continue_or_abort {
            ( $response_bytes:ident ) => {{
                let err_response = serde_json::from_slice::<DeviceAccessTokenErrorResponse>(&$response_bytes)?;
                if err_response.error.should_continue_to_poll() {
                    debug!(error_code=?err_response.error,interval, "Continue to poll");

                    let interval = interval.unwrap_or(5);

                    tokio::time::sleep(Duration::from_secs(interval as u64)).await;
                } else {
                    anyhow::bail!(
                        "Failed to authenticate. authorization server respond with {err_response:?}"
                    )
                }
            }};
        }

        let response = loop {
            let response = self
                .client
                .post(Self::TOKEN_ENDPOINT)
                .header(http::header::ACCEPT, "application/json")
                .form(&DeviceAccessTokenRequest::new(&device_code, self.client_id))
                .send()
                .await?;

            match response.status() {
                StatusCode::OK => {
                    let full = response.bytes().await?;
                    match serde_json::from_slice::<DeviceAccessTokenResponse>(&full) {
                        Ok(response) => break response,
                        Err(_) => continue_or_abort!(full),
                    }
                }
                StatusCode::BAD_REQUEST => {
                    let full = response.bytes().await?;
                    continue_or_abort!(full)
                }
                other => {
                    let error_msg = response.text().await.unwrap_or_default();
                    anyhow::bail!("Failed to authenticate. authorization server respond with {other} {error_msg}")
                }
            }
        };

        Ok(response)
    }
}
