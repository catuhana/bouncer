use std::sync::Arc;

use event_handler::{EventExt as _, EventHandler};
use secrecy::{ExposeSecret, SecretString};
use twilight_gateway::{EventTypeFlags, Intents, Shard, ShardId, StreamExt};
use twilight_http::Client as HttpClient;

pub mod event_handler;

pub struct Client {
    shard: Shard,
    pub http: Arc<HttpClient>,
    event_handler: Box<dyn EventHandler>,
}

pub struct ClientBuilder {
    http: HttpClient,
    shards: ShardId,
    intents: Intents,
    event_handler: Option<Box<dyn EventHandler>>,
}

impl Client {
    #[must_use]
    pub fn builder(token: &SecretString) -> ClientBuilder {
        ClientBuilder {
            http: HttpClient::new(token.expose_secret().to_owned()),
            shards: ShardId::ONE,
            intents: Intents::empty(),
            event_handler: None,
        }
    }

    pub async fn start(&mut self, event_types: EventTypeFlags) {
        while let Some(event) = self.shard.next_event(event_types).await {
            match event {
                Ok(event) => event.dispatch(&*self.event_handler).await,
                Err(error) => {
                    // TODO: We probably want to handle this.
                    eprintln!("Error: {error:?}");
                }
            }
        }
    }
}

impl ClientBuilder {
    #[must_use]
    /// # Panics
    ///
    /// Panics if [`ClientBuilder::event_handler`] is not set with
    /// [`ClientBuilder::event_handler`].
    pub fn build(self) -> Client {
        let http = Arc::new(self.http);
        let shard = Shard::new(
            self.shards,
            http.token()
                .expect("HTTP client doesn't have token")
                .to_owned(),
            self.intents,
        );
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
