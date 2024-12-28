use bouncer_framework::command::{Command, CommandExecuteError, CommandExecutor};
use bouncer_macros::BouncerCommand;

#[derive(Debug, BouncerCommand)]
#[command(name = "meow", description = "Meow!")]
pub struct MeowCommand {
    #[option(description = "MEOWW :3:3")]
    test_option: bool,
    #[option(description = "How many times to repeat the meow!")]
    repeat: Option<i64>,
}

#[async_trait::async_trait]
impl CommandExecutor for MeowCommand {
    async fn execute(&self) -> Result<(), CommandExecuteError> {
        tracing::info!(
            "Executing `meow` command :: {}",
            "Meow!".repeat(self.repeat.unwrap_or_else(|| 1) as usize)
        );
        tracing::info!("Test option :: {}", self.test_option);
        tracing::info!("Data :: {:?}", self);
        Ok(())
    }
}

impl Command for MeowCommand {}
