use anyhow::{self, Context};
use tokio::net::TcpListener;

/// init the logger
/// Log level: `info` by default, use `LOG` env to change
fn init_log() -> Result<(), log::SetLoggerError> {
    let mut builder = pretty_env_logger::formatted_builder();
    #[allow(clippy::or_fun_call)]
    builder.parse_filters(&std::env::var("LOG").unwrap_or("info".to_string()));
    builder.try_init()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log()?;

    let socket = TcpListener::bind("127.0.0.1:0")
        .await
        .context("failed to bind adress")?;
    log::info!("listening on {}", socket.local_addr()?);
    Ok(())
}
