use bouncer_framework::{Client, Context, EventHandler};
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

    client.start().await;

    Ok(())
}

struct Events;

#[async_trait::async_trait]
impl EventHandler for Events {
    async fn ready(&self, _: Context, ready: Box<Ready>) {
        tracing::info!("Bouncer is ready as {}", ready.user.name);
    }
}
