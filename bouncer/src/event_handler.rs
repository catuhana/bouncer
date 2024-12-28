use bouncer_framework::{command::Command as _, Context, EventHandler};
use twilight_model::{
    application::interaction::InteractionData,
    gateway::payload::incoming::{InteractionCreate, Ready},
};

use crate::commands;

pub struct Events;

impl Events {
    pub async fn register_commands(context: Context) -> Result<(), EventsError> {
        let application_id = context
            .http
            .current_user_application()
            .await?
            .model()
            .await?
            .id;

        context
            .http
            .interaction(application_id)
            .set_global_commands(&commands::Commands::all_commands())
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for Events {
    async fn ready(&self, context: Context, ready: Box<Ready>) {
        tracing::info!("Bouncer is ready as {}", ready.user.name);

        match Self::register_commands(context).await {
            // TODO: Maybe list registered commands.
            Ok(()) => tracing::info!("commands registered"),
            Err(error) => tracing::error!("{}", error),
        }
    }

    async fn interaction_create(&self, _: Context, interaction: Box<InteractionCreate>) {
        match interaction.data.as_ref() {
            Some(InteractionData::ApplicationCommand(command)) => {
                match commands::Commands::parse_from_command_name(&command.name, &command.options) {
                    Ok(commands) => match commands {
                        commands::Commands::Meow(command) => {
                            // TODO: Handle error.
                            command.execute().await.unwrap();
                        }
                    },
                    Err(error) => tracing::error!("{}", error),
                }
            }
            interaction => {
                tracing::warn!("Unhandled interaction type {:?}", interaction);
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventsError {
    #[error("An HTTP error occurred: {0}")]
    TwilightHttpError(#[from] twilight_http::Error),
    #[error("An error occurred while deserialising a model: {0}")]
    TwilightModelDeserialiseError(#[from] twilight_http::response::DeserializeBodyError),
}
