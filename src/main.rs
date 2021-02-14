// mod server;
mod packet;
mod server;

/// init the logger
/// Log level: `info` by default, use `LOG` env to change
fn init_log() -> Result<(), log::SetLoggerError> {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.parse_filters(&std::env::var("LOG").unwrap_or_else(|_| "info".to_string()));
    builder.try_init()
    // TODO: Return `builder`
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log()?;

    let mut socket = server::ChatServer::bind(8000).await?;

    socket.run().await;

    Ok(())
}
