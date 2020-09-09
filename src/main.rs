use anyhow::{self, Context};
use tokio::{net, prelude::*, stream::StreamExt, task};

struct ChatServer {
    listener: net::TcpListener,
}

impl ChatServer {
    async fn handle_connections(stream: &mut net::TcpStream) {
        let mut buf = bytes::BytesMut::with_capacity(64);
        let addr = stream.peer_addr().unwrap();

        log::info!("{}: Got connection", addr);
        loop {
            let res = stream.read_buf(&mut buf).await;
            let res = res.or(stream.write_buf(&mut buf).await);
            if let Ok(0) | Err(_) = res {
                break;
            }
        }
        log::info!("{}: Connection closed", addr);
    }

    async fn run(&mut self) {
        while let Some(stream) = self.listener.incoming().next().await {
            let mut stream = match stream {
                Ok(s) => s,
                Err(err) => {
                    log::error!("{}", err);
                    continue;
                }
            };
            let _handle = task::spawn(async move {
                ChatServer::handle_connections(&mut stream).await;
            });
        }
    }

    async fn bind(port: u16) -> anyhow::Result<Self> {
        let res = Self {
            listener: net::TcpListener::bind(("127.0.0.1", port))
                .await
                .context("failed to bind adress")?,
        };
        log::info!("listening on {}", res.listener.local_addr()?);
        Ok(res)
    }
}

/// init the logger
/// Log level: `info` by default, use `LOG` env to change
fn init_log() -> Result<(), log::SetLoggerError> {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.parse_filters(&std::env::var("LOG").unwrap_or_else(|_| "info".to_string()));
    builder.try_init()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log()?;

    let mut socket = ChatServer::bind(8000u16).await?;

    socket.run().await;

    Ok(())
}
