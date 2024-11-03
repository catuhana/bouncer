use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The token of the Discord bot.
    pub token: SecretString,
}
