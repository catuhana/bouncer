use bouncer_framework::Client;

use crate::event_handler::Events;

mod commands;
mod event_handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: Create a workspace for this.
    tracing_subscriber::fmt::init();

    let cli = bouncer_cli::Cli::parse_and_validate()?;
    let config = bouncer_config::Config::parse(&cli.config)?;

    let mut client = Client::builder(&config.discord.token)
        .event_handler(Events)
        .try_build()?;

    client.start().await;

    Ok(())
}
