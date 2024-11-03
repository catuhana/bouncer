use std::path::Path;

use figment::{
    providers::{Env, Format as _, Yaml},
    Figment,
};
use serde::Deserialize;

pub mod discord;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Discord configuration.
    pub discord: discord::Config,
}

/// Parse bouncer configuration from a file and environment variables.
pub fn parse_config(config_path: impl AsRef<Path>) -> anyhow::Result<Config> {
    Figment::new()
        .merge(Yaml::file(config_path))
        .merge(Env::prefixed("BOUNCER_").split("__"))
        .extract::<Config>()
        .map_err(anyhow::Error::from)
}
