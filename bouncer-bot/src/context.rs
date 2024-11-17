use std::sync::Arc;

use tokio::sync::Mutex;
use twilight_gateway::Shard;
use twilight_http::Client as HttpClient;

pub struct Context {
    pub shard: Arc<Mutex<Shard>>,
    pub http: Arc<HttpClient>,
}

impl Context {
    pub fn new(shard: Arc<Mutex<Shard>>, http: Arc<HttpClient>) -> Self {
        Self { shard, http }
    }
}
