use bouncer_framework::command::{Command, CommandExecuteError};
use bouncer_macros::BouncerCommand;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, RoleMarker, UserMarker},
};

#[derive(Debug, BouncerCommand)]
#[command(name = "meow", description = "Meow!")]
pub struct MeowCommand {
    #[option(description = "Test string option")]
    _string: String,
    #[option(description = "Test integer option")]
    _integer: i64,
    #[option(description = "Test boolean option")]
    _boolean: bool,
    #[option(description = "Test user option")]
    _user: Id<UserMarker>,
    #[option(description = "Test channel option")]
    _channel: Id<ChannelMarker>,
    #[option(description = "Test role option")]
    _role: Id<RoleMarker>,
}

#[async_trait::async_trait]
impl Command for MeowCommand {
    async fn execute(&self) -> Result<(), CommandExecuteError> {
        tracing::info!("data :: {:?}", self);
        Ok(())
    }
}
