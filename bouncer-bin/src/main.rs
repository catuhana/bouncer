#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: Create a workspace for this.
    tracing_subscriber::fmt::init();

    let cli = bouncer_cli::Cli::parse_and_validate()?;
    let config = bouncer_config::Config::parse(&cli.config)?;

    dbg!(config);

    Ok(())
}
