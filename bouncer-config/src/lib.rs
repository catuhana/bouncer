use std::path::Path;

use figment::{
    Figment,
    providers::{Env, Format as _, Yaml},
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

#[cfg(test)]
mod tests {
    use figment::Jail;
    use secrecy::ExposeSecret as _;

    use crate::{Config, ConfigParseError};

    #[test]
    fn test_parse_valid_config() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "config.yaml",
                r"
                discord:
                    token: meow
                ",
            )?;

            let config = Config::parse("config.yaml").unwrap();
            assert_eq!(config.discord.token.expose_secret(), "meow");

            Ok(())
        });
    }

    #[test]
    fn test_if_env_overrides_file() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "config.yaml",
                r"
                discord:
                    token: meow
                ",
            )?;

            jail.set_env("BOUNCER_DISCORD__TOKEN", "mrrp");

            let config = Config::parse("config.yaml").unwrap();
            assert_eq!(config.discord.token.expose_secret(), "mrrp");

            Ok(())
        });
    }

    #[test]
    fn test_missing_config_file() {
        let result = Config::parse("nonexistent.yaml");

        assert!(matches!(result, Err(ConfigParseError::FigmentExtract(_))));
    }

    #[test]
    fn test_parse_invalid_config() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "config.yaml",
                r"
                discord: {
                  token: [invalid
                ",
            )?;

            let result = Config::parse("config.yaml");
            assert!(matches!(result, Err(ConfigParseError::FigmentExtract(_))));

            Ok(())
        });
    }
}
