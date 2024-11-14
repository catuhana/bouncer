use bouncer_bot::{event_handler::EventHandler, Client};
use twilight_model::gateway::payload::incoming::Ready;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: Create a workspace for this.
    tracing_subscriber::fmt::init();

    let cli = bouncer_cli::Cli::parse_and_validate()?;
    let config = bouncer_config::Config::parse(&cli.config)?;

    let mut client = Client::builder(&config.discord.token)
        .event_handler(Events)
        .build();

    // To make clippy shut for now for `Client::http` not being used.
    dbg!(
        client
            .http
            .current_user_application()
            .await?
            .model()
            .await?
            .id
    );

    client.start().await;

    Ok(())
}

struct Events;

#[async_trait::async_trait]
impl EventHandler for Events {
    async fn ready(&self, ready: Box<Ready>) {
        tracing::info!("Bouncer is ready as {:?}", ready.user.name);
    }
}
