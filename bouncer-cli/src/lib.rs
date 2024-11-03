use std::path::PathBuf;

use clap::Parser;

/// CLI options.
#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub config: PathBuf,
}

impl Cli {
    /// Parses and validates CLI input.
    ///
    /// # Errors
    ///
    /// When CLI validation fails, returns [`CliParseError::Validate`].
    pub fn parse_and_validate() -> Result<Self, CliParseError> {
        Self::validate(Self::parse()).map_err(From::from)
    }

    /// Validates CLI input.
    ///
    /// # Errors
    ///
    /// When one of the `_validate` suffixed functions (e.g. [`Self::_config_validate`])
    /// fail, returns [`ValidationError::Config`].
    fn validate(self) -> Result<Self, ValidationError> {
        Self::_config_validate(&self.config.to_string_lossy())?;

        Ok(self)
    }

    /// Validates the [`Cli::config`] option.
    ///
    /// # Errors
    ///
    /// If specified `path` doesn't exist, returns [`ConfigValidationError::DoesNotExist`]
    /// error.
    fn _config_validate(path: &str) -> Result<PathBuf, ConfigValidationError> {
        let path = PathBuf::from(path);
        if !path.exists() {
            return Err(ConfigValidationError::DoesNotExist(
                path.to_string_lossy().to_string(),
            ));
        }

        Ok(path)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CliParseError {
    #[error(transparent)]
    Validate(#[from] ValidationError),
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error(transparent)]
    Config(#[from] ConfigValidationError),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigValidationError {
    #[error("Specified config file does not exist: {0}")]
    DoesNotExist(String),
}

// TODO: Add tests.
