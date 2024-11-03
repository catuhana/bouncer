use secrecy::SecretString;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// The token of the Discord bot.
    pub token: SecretString,
}
