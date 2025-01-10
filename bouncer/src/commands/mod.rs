use bouncer_framework::command::{
    CommandData as _, CommandDataError, CommandOptions as _, CommandOptionsError,
};
use twilight_model::application::{
    command::Command, interaction::application_command::CommandDataOption,
};

pub mod meow;

pub enum Commands {
    Meow(meow::MeowCommand),
}

impl Commands {
    pub fn parse_from_command_name(
        name: &str,
        options: &[CommandDataOption],
    ) -> Result<Self, CommandsError> {
        match name {
            meow::MeowCommand::COMMAND_NAME => {
                Ok(Self::Meow(meow::MeowCommand::parse_options(options)?))
            }
            _ => Err(CommandsError::CommandNameParseError(name.to_string())),
        }
    }

    pub fn all_commands() -> Result<Vec<Command>, CommandDataError> {
        Ok(vec![meow::MeowCommand::command()?])
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandsError {
    #[error("Could not find command {0}")]
    CommandNameParseError(String),
    #[error(transparent)]
    CommandOptionsError(#[from] CommandOptionsError),
}
