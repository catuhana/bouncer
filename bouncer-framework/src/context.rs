use std::sync::Arc;

use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client as HttpClient;

#[derive(Debug)]
pub struct Context {
    pub http: Arc<HttpClient>,
    pub cache: Arc<InMemoryCache>,
}

impl Context {
    pub const fn new(http: Arc<HttpClient>, cache: Arc<InMemoryCache>) -> Self {
        Self { http, cache }
    }
}
