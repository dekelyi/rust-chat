use anyhow::{self, Context};
use tokio::{net, prelude::*, stream::StreamExt, task};

/// a chat server instance
pub struct ChatServer {
    listener: net::TcpListener,
}

impl ChatServer {
    /// handle a new connection
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

    /// run the main event loop
    pub async fn run(&mut self) {
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

    /// Create a new instance of a bound server
    pub async fn bind(port: u16) -> anyhow::Result<Self> {
        let res = Self {
            listener: net::TcpListener::bind(("127.0.0.1", port))
                .await
                .context("failed to bind adress")?,
        };
        log::info!("listening on {}", res.listener.local_addr()?);
        Ok(res)
    }
}
