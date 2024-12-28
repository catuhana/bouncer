use std::sync::Arc;

use secrecy::{ExposeSecret as _, SecretString};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;

use crate::{
    context::Context,
    event_handler::{EventExt as _, EventHandler},
};

pub struct Client {
    shard: Shard,
    http: Arc<HttpClient>,
    cache: Arc<InMemoryCache>,
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
        loop {
            match self
                .shard
                .next_event(self.event_handler.used_event_flags())
                .await
            {
                Some(Ok(event)) => {
                    self.cache.update(&event);
                    event
                        .dispatch(self.create_context(), &*self.event_handler)
                        .await;
                }
                Some(Err(error)) => {
                    tracing::error!(source = ?error, "error receiving event");
                }
                None => break,
            }
        }
    }

    fn create_context(&self) -> Context {
        Context::new(self.http.clone(), self.cache.clone())
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
        let shard = Shard::new(
            self.shard_id,
            http.token()
                .expect("HTTP client doesn't have token")
                .to_owned(),
            self.intents,
        );
        let cache = Arc::new(InMemoryCache::new());
        let event_handler = self.event_handler.expect("event handler not set");

        Client {
            shard,
            http,
            cache,
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
