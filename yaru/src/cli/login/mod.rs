use clap::{Args, Subcommand};

mod oauth;

/// login to authenticate client
#[derive(Args, Debug)]
pub struct LoginCommand {
    #[command(subcommand)]
    protocol: LoginProtocol,
}

#[derive(Subcommand, Debug)]
enum LoginProtocol {
    #[command(name = "oauth")]
    OAuth(oauth::OAuthLoginCommand),
}

impl LoginCommand {
    pub(super) async fn run(self) -> anyhow::Result<()> {
        let Self { protocol } = self;

        match protocol {
            LoginProtocol::OAuth(cmd) => cmd.run().await?,
        }

        Ok(())
    }
}
