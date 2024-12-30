use twilight_http::Client as HttpClient;
use twilight_model::{
    application::interaction::Interaction, http::interaction::InteractionResponse,
};

#[async_trait::async_trait]
pub trait InteractionExt {
    async fn test(
        &self,
        http_client: &HttpClient,
        response: InteractionResponse,
    ) -> Result<(), InteractionExtError>;
}

// TODO: Custom `response` type for messages.
#[async_trait::async_trait]
impl InteractionExt for Interaction {
    async fn test(
        &self,
        http_client: &HttpClient,
        response: InteractionResponse,
    ) -> Result<(), InteractionExtError> {
        let application_id = self.application_id;

        http_client
            .interaction(application_id)
            .create_response(self.id, &self.token, &response)
            .await?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InteractionExtError {
    #[error(transparent)]
    HttpError(#[from] twilight_http::Error),
}
