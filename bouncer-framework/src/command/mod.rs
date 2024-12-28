use twilight_model::application::{
    command::{Command as TwilightCommand, CommandType},
    interaction::application_command::{CommandDataOption, CommandOptionValue},
};
use twilight_util::builder::command::CommandBuilder;

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

pub trait CommandOptions {
    fn parse_options(options: &[CommandDataOption]) -> Result<Self, CommandOptionsError>
    where
        Self: Sized;
}

#[async_trait::async_trait]
pub trait CommandExecutor {
    // TODO: Have `interaction` and `ctx` as arguments.
    async fn execute(&self) -> Result<(), CommandExecuteError>;
}

pub trait Command: CommandData + CommandOptions + CommandExecutor {}

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
