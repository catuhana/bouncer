#[async_trait::async_trait]
pub trait Command {
    const COMMAND_NAME: &'static str;
    const COMMAND_DESCRIPTION: &'static str;

    async fn execute(&self) -> Result<(), CommandExecuteError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CommandExecuteError {
    #[error("An error occurred while executing the command: {0}")]
    CommandError(#[from] anyhow::Error),
}
