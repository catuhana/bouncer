use std::sync::Arc;

use secrecy::{ExposeSecret as _, SecretString};
use tokio::sync::Mutex;
use twilight_gateway::{Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;

use crate::{
    context::Context,
    event_handler::{EventExt as _, EventHandler},
};

pub struct Client {
    shard: Arc<Mutex<Shard>>,
    http: Arc<HttpClient>,
    event_handler: Box<dyn EventHandler>,
}

pub struct ClientBuilder {
    http: HttpClient,
    shard_id: ShardId,
    intents: Intents,
    event_handler: Option<Box<dyn EventHandler>>,
}

impl Client {
    #[must_use]
    pub fn builder(token: &SecretString) -> ClientBuilder {
        ClientBuilder {
            http: HttpClient::new(token.expose_secret().to_owned()),
            shard_id: ShardId::ONE,
            intents: Intents::empty(),
            event_handler: None,
        }
    }

    pub async fn start(&mut self) {
        while let Some(event) = self
            .shard
            .lock()
            .await
            .next_event(self.event_handler.used_event_flags())
            .await
        {
            let Ok(event) = event else {
                tracing::error!(source = ?event.unwrap_err(), "error receiving event");

                continue;
            };

            event
                .dispatch(self.create_context(), &*self.event_handler)
                .await;
        }
    }

    fn create_context(&self) -> Context {
        Context::new(self.shard.clone(), self.http.clone())
    }
}

impl ClientBuilder {
    /// # Panics
    ///
    /// Panics if [`ClientBuilder::event_handler`] is not set.
    // TODO: Maybe errors here should be handled properly?
    #[must_use]
    pub fn build(self) -> Client {
        let http = Arc::new(self.http);
        let shard = Arc::new(Mutex::new(Shard::new(
            self.shard_id,
            http.token()
                .expect("HTTP client doesn't have token")
                .to_owned(),
            self.intents,
        )));
        let event_handler = self.event_handler.expect("event handler not set");

        Client {
            shard,
            http,
            event_handler,
        }
    }

    #[must_use]
    pub const fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents;

        self
    }

    #[must_use]
    pub fn event_handler(mut self, event_handler: impl EventHandler + 'static) -> Self {
        self.event_handler = Some(Box::new(event_handler));

        self
    }
}
