use std::path::Path;

use figment::{
    providers::{Env, Format as _, Yaml},
    Figment,
};

pub mod discord;

/// Configuration options.
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// Discord configuration options.
    pub discord: discord::Config,
}

impl Config {
    /// Parse bouncer configuration from a file and environment variables.
    ///
    /// # Errors
    ///
    /// When [`Figment::extract`] fails, returns [`ConfigParseError::FigmentExtract`]
    /// error.
    pub fn parse(config_path: impl AsRef<Path>) -> Result<Self, ConfigParseError> {
        Figment::new()
            .merge(Yaml::file(config_path))
            .merge(Env::prefixed("BOUNCER_").split("__"))
            .extract()
            .map_err(From::from)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigParseError {
    #[error(transparent)]
    FigmentExtract(#[from] figment::Error),
}

// TODO: Add tests.
