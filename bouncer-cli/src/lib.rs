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
        let parsed = Self::parse();
        parsed.validate()?;

        Ok(parsed)
    }

    /// Validates CLI input.
    ///
    /// # Errors
    ///
    /// When one of the `_validate` suffixed functions (e.g. [`Self::_config_validate`])
    /// fail, returns [`ValidationError::Config`].
    fn validate(&self) -> Result<(), ValidationError> {
        Self::_config_validate(&self.config.to_string_lossy())?;
        Ok(())
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
#[cfg_attr(test, derive(PartialEq))]
pub enum ValidationError {
    #[error(transparent)]
    Config(#[from] ConfigValidationError),
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ConfigValidationError {
    #[error("Specified config file does not exist: {0}")]
    DoesNotExist(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_parse() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let args = vec!["test", "--config", path];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.config, PathBuf::from(path));
    }

    #[test]
    fn test_cli_validation_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let cli = Cli {
            config: PathBuf::from(path),
        };

        assert!(cli.validate().is_ok());
    }

    #[test]
    fn test_config_validation_file_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let result = Cli::_config_validate(path);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from(path));
    }

    #[test]
    fn test_config_validation_file_does_not_exist() {
        let non_existent_path = "/path/that/does/not/exist/config.yaml";

        let result = Cli::_config_validate(non_existent_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ConfigValidationError::DoesNotExist(non_existent_path.to_string())
        );
    }

    #[test]
    fn test_cli_parse_and_validate() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        let args = vec!["test", "--config", path];
        let result = Cli::try_parse_from(args).unwrap().validate();

        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_parse_and_validate_non_existent_file() {
        let non_existent_path = "/path/that/does/not/exist/config.yaml";

        let args = vec!["test", "--config", non_existent_path];
        let result = Cli::try_parse_from(args).unwrap().validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ValidationError::Config(ConfigValidationError::DoesNotExist(
                non_existent_path.to_string()
            ))
        );
    }
}
