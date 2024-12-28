use bouncer_framework::command::{Command, CommandExecuteError};
use bouncer_macros::BouncerCommand;
use twilight_model::id::{
    marker::{ChannelMarker, RoleMarker, UserMarker},
    Id,
};

#[derive(Debug, BouncerCommand)]
#[command(name = "meow", description = "Meow!")]
pub struct MeowCommand {
    #[option(description = "Test string option")]
    _string_option: String,
    #[option(description = "Test integer option")]
    _integer_option: i64,
    #[option(description = "Test boolean option")]
    _boolean_option: bool,
    #[option(description = "Test user option")]
    _user_option: Id<UserMarker>,
    #[option(description = "Test channel option")]
    _channel_option: Id<ChannelMarker>,
    #[option(description = "Test role option")]
    _role_option: Id<RoleMarker>,
}

#[async_trait::async_trait]
impl Command for MeowCommand {
    async fn execute(&self) -> Result<(), CommandExecuteError> {
        tracing::info!("data :: {:?}", self);
        Ok(())
    }
}
