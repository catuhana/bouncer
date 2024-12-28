use bouncer_framework::command::{CommandData, CommandOptions, CommandOptionsError};
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
                Ok(Commands::Meow(meow::MeowCommand::parse_options(options)?))
            }
            _ => Err(CommandNameParseError {
                command_name: name.to_string(),
            }
            .into()),
        }
    }

    pub fn all_commands() -> Vec<Command> {
        vec![meow::MeowCommand::command()]
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandsError {
    #[error(transparent)]
    CommandNameParseError(#[from] CommandNameParseError),
    #[error(transparent)]
    CommandOptionsError(#[from] CommandOptionsError),
}

#[derive(Debug, thiserror::Error)]
pub struct CommandNameParseError {
    pub command_name: String,
}

impl ::std::fmt::Display for CommandNameParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Could not find command {}", self.command_name)
    }
}
