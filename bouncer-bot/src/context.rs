use std::sync::Arc;

use tokio::sync::Mutex;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::Shard;
use twilight_http::Client as HttpClient;

pub struct Context {
    pub shard: Arc<Mutex<Shard>>,
    pub http: Arc<HttpClient>,
    pub cache: Arc<InMemoryCache>,
}

impl Context {
    pub fn new(shard: Arc<Mutex<Shard>>, http: Arc<HttpClient>, cache: Arc<InMemoryCache>) -> Self {
        Self { shard, http, cache }
    }
}
