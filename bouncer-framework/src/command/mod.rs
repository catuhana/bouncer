use twilight_model::application::{
    command::{Command as TwilightCommand, CommandType},
    interaction::{
        Interaction,
        application_command::{CommandDataOption, CommandOptionValue},
    },
};
use twilight_util::builder::command::CommandBuilder;

use crate::{Context, exts::interaction::InteractionExtError};

pub trait CommandData {
    const COMMAND_NAME: &'static str;
    const COMMAND_DESCRIPTION: &'static str;

    fn command() -> TwilightCommand;
    fn command_builder() -> CommandBuilder {
        CommandBuilder::new(
            Self::COMMAND_NAME,
            Self::COMMAND_DESCRIPTION,
            CommandType::ChatInput,
        )
    }
}

pub trait CommandOptions: Sized {
    /// # Errors
    ///
    /// Returns a `CommandOptionsError` if the options could not be parsed.
    fn parse_options(options: &[CommandDataOption]) -> Result<Self, CommandOptionsError>;
}

#[async_trait::async_trait]
pub trait Command: CommandData + CommandOptions {
    async fn execute(
        &self,
        context: &Context,
        interaction: &Interaction,
    ) -> Result<(), CommandExecuteError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    CommandExecuteError(#[from] CommandExecuteError),
    #[error(transparent)]
    CommandOptionsError(#[from] CommandOptionsError),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandExecuteError {
    #[error("An error occurred while executing the command: {0}")]
    CommandError(#[from] anyhow::Error),
    #[error(transparent)]
    InteractionError(#[from] InteractionExtError),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandOptionsError {
    // #[error("Failed to parse command options: {0}")]
    // ParseError(String),
    #[error("Unexpected option type for {0}")]
    UnexpectedOptionType(String, CommandOptionValue),
    #[error("Missing required option {0}")]
    MissingRequiredOption(String),
}
