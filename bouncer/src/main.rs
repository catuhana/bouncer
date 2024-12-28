use bouncer_framework::{
    command::{CommandData as _, CommandExecutor as _, CommandOptions as _},
    Client, Context, EventHandler,
};
use twilight_model::{
    application::interaction::InteractionData,
    gateway::payload::incoming::{InteractionCreate, Ready},
};

mod commands;

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

impl Events {
    pub async fn register_commands(context: Context) {
        // TODO: Handle errors.
        let application_id = context
            .http
            .current_user_application()
            .await
            .unwrap()
            .model()
            .await
            .unwrap()
            .id;

        dbg!(commands::meow::MeowCommand::command());

        context
            .http
            .interaction(application_id)
            .set_global_commands(&vec![commands::meow::MeowCommand::command()])
            .await
            .unwrap();
    }
}

#[async_trait::async_trait]
impl EventHandler for Events {
    async fn ready(&self, context: Context, ready: Box<Ready>) {
        tracing::info!("Bouncer is ready as {}", ready.user.name);
        Self::register_commands(context).await;
    }

    async fn interaction_create(&self, _: Context, interaction: Box<InteractionCreate>) {
        // tracing::info!("Interaction received: {:?}", interaction);

        match interaction.data.as_ref() {
            Some(InteractionData::ApplicationCommand(command)) => {
                let command_name = command.name.as_str();

                match command_name {
                    commands::meow::MeowCommand::COMMAND_NAME => {
                        let command = commands::meow::MeowCommand::parse_options(&command.options);

                        match command {
                            Ok(command) => {
                                command.execute().await.unwrap();
                            }
                            Err(error) => {
                                tracing::error!("Failed to parse command options: {}", error);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
