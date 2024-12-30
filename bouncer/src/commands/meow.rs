use bouncer_framework::{
    Context,
    command::{Command, CommandExecuteError},
    exts::interaction::InteractionExt,
};
use bouncer_macros::BouncerCommand;
use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{
        Id,
        marker::{ChannelMarker, RoleMarker, UserMarker},
    },
};
use twilight_util::builder::InteractionResponseDataBuilder;

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
    async fn execute(
        &self,
        context: &Context,
        interaction: &Interaction,
    ) -> Result<(), CommandExecuteError> {
        interaction
            .test(&context.http, InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseDataBuilder::new().content("uwu").build()),
            })
            .await?;

        Ok(())
    }
}
