use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub config: PathBuf,
}

impl Cli {
    pub fn parse_and_validate() -> Result<Self, CliParseError> {
        Self::validate(&Self::parse()).map_err(From::from)
    }

    fn validate(&self) -> Result<Self, ValidationError> {
        Self::_config_validate(&self.config.to_string_lossy())?;

        Ok(Self::parse())
    }

    fn _config_validate(value: &str) -> Result<PathBuf, ConfigValidationError> {
        let path = PathBuf::from(value);
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
