#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: Create a workspace for this.
    tracing_subscriber::fmt::init();

    // TODO: Create a workspace for its argument (CLI).
    let config = bouncer_config::parse_config("config.yaml")?;

    dbg!(config);

    Ok(())
}
