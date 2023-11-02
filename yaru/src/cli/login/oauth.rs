use clap::Args;
use tracing::{debug, info};

use crate::auth;

#[derive(Args,Debug)]
pub struct OAuthLoginCommand {
    /// Authorization server
    #[arg(
    value_enum, 
    long, 
    default_value_t = AuthorizationServer::Github,
    visible_alias = "auth",
    )]
    authorization_server: AuthorizationServer,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, clap::ValueEnum)]
enum AuthorizationServer {
    Github,
}

impl OAuthLoginCommand {
    pub(super) async fn run(self) -> anyhow::Result<()> {
        let Self {
            authorization_server
        } = self;

        debug!(?authorization_server, "Login...");

        match authorization_server {
            AuthorizationServer::Github => {
                let response = auth::github::DeviceFlow::new().device_flow().await?; 

                // TODO: store in cache storage
                debug!("{response:?}");
                info!("Successfully logined");
            }
        }

        Ok(())
    }
}
