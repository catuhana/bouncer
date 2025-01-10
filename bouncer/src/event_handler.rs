use bouncer_framework::{
    Context, EventHandler,
    command::{Command as _, CommandDataError},
};
use twilight_http::response::DeserializeBodyError;
use twilight_model::{
    application::interaction::InteractionData,
    gateway::payload::incoming::{InteractionCreate, Ready},
};

use crate::commands;

pub struct Events;

impl Events {
    pub async fn register_commands(context: Context) -> Result<Vec<String>, EventsError> {
        let application_id = context
            .http
            .current_user_application()
            .await?
            .model()
            .await?
            .id;

        let registered_commands = context
            .http
            .interaction(application_id)
            .set_global_commands(&commands::Commands::all_commands()?)
            .await?
            .model()
            .await?;

        Ok(registered_commands
            .iter()
            .map(|command| command.name.clone())
            .collect())
    }
}

#[async_trait::async_trait]
impl EventHandler for Events {
    async fn ready(&self, context: Context, ready: Box<Ready>) {
        tracing::info!("Bouncer is ready as {}", ready.user.name);

        match Self::register_commands(context).await {
            Ok(registered_command_names) => {
                tracing::info!("registered command names: {registered_command_names:?}");
            }
            Err(error) => tracing::error!("{}", error),
        }
    }

    async fn interaction_create(&self, context: Context, interaction: Box<InteractionCreate>) {
        match interaction.data.as_ref() {
            Some(InteractionData::ApplicationCommand(command)) => {
                let command = match commands::Commands::parse_from_command_name(
                    &command.name,
                    &command.options,
                ) {
                    Ok(cmd) => cmd,
                    Err(error) => {
                        tracing::error!(?error, "failed to parse command");
                        return;
                    }
                };

                if let Err(error) = match command {
                    commands::Commands::Meow(command) => {
                        command.execute(&context, &interaction.0).await
                    }
                } {
                    tracing::error!(?error, "failed to handle command");
                }
            }
            interaction => {
                tracing::warn!("unhandled interaction type {:?}", interaction);
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventsError {
    #[error("An HTTP error occurred: {0}")]
    TwilightHttp(#[from] twilight_http::Error),
    #[error("An error occurred while deserialising a model: {0}")]
    TwilightModelDeserialise(#[from] DeserializeBodyError),
    #[error(transparent)]
    CommandData(#[from] CommandDataError),
}
