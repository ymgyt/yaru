use clap::{Parser, Subcommand};

mod login;

#[derive(Parser, Debug)]
#[command(
    version,
    propagate_version = true,
    disable_help_subcommand = true,
    help_expected = true,
    infer_subcommands = true
)]
pub struct YaruApp {
    #[command(subcommand)]
    pub command: Option<Command>,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    Login(login::LoginCommand),
}

pub fn parse() -> YaruApp {
    YaruApp::parse()
}

impl YaruApp {
    pub async fn run(self) -> anyhow::Result<()> {
        let Self { command } = self;

        match command {
            Some(Command::Login(cmd)) => cmd.run().await,
            None => unimplemented!(),
        }
    }
}
