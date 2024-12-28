use bouncer_framework::command::{Command, CommandExecuteError, CommandExecutor};
use bouncer_macros::BouncerCommand;
use twilight_model::id::{
    marker::{ChannelMarker, RoleMarker, UserMarker},
    Id,
};

#[derive(Debug, BouncerCommand)]
#[command(name = "meow", description = "Meow!")]
pub struct MeowCommand {
    #[option(description = "Test string option")]
    test_string_option: String,
    #[option(description = "Test integer option")]
    test_integer_option: i64,
    #[option(description = "Test boolean option")]
    test_boolean_option: bool,
    #[option(description = "Test user option")]
    test_user_option: Id<UserMarker>,
    #[option(description = "Test channel option")]
    test_channel_option: Id<ChannelMarker>,
    #[option(description = "Test role option")]
    test_role_option: Id<RoleMarker>,
}

#[async_trait::async_trait]
impl CommandExecutor for MeowCommand {
    async fn execute(&self) -> Result<(), CommandExecuteError> {
        tracing::info!("data :: {:?}", self);
        Ok(())
    }
}

impl Command for MeowCommand {}
