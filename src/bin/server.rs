use anyhow::{self, Context};
use std::sync::Arc;
use tokio::{net, prelude::*, stream::StreamExt, sync::Mutex, task};

type Stream = Arc<Mutex<net::TcpStream>>;
type Connection = (Stream, task::JoinHandle<()>);

struct ChatServer {
    listener: net::TcpListener,
    connected: Vec<Connection>,
}

impl ChatServer {
    async fn handle_connections(stream: &mut Stream) -> anyhow::Result<()> {
        let mut buf = bytes::BytesMut::with_capacity(64);
        let addr = stream.lock().await.peer_addr()?;

        log::info!("Got connection from {}", addr);
        loop {
            if let Ok(0) | Err(_) = stream.lock().await.read_buf(&mut buf).await {
                break;
            }
            if stream.lock().await.write_buf(&mut buf).await.is_err() {
                break;
            }
        }
        log::info!("Closed connection with: {}", addr);

        Ok(())
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(stream) = self.listener.next().await {
            let stream = Arc::new(Mutex::new(stream?));
            let handle = {
                let mut stream = stream.clone();
                task::spawn(async move {
                    ChatServer::handle_connections(&mut stream).await.unwrap();
                })
            };
            self.connected.push((stream, handle));
        }
        Ok(())
    }

    async fn close(&mut self) -> anyhow::Result<()> {
        log::info!("Shuting the server down");
        for (stream, handle) in &mut self.connected {
            stream.lock().await.shutdown(std::net::Shutdown::Both)?;
            handle.await?;
        }
        Ok(())
    }

    async fn bind(port: u16) -> anyhow::Result<Self> {
        let res = Self {
            listener: net::TcpListener::bind(("127.0.0.1", port))
                .await
                .context("failed to bind adress")?,
            connected: Default::default(),
        };
        log::info!("listening on {}", res.listener.local_addr()?);
        Ok(res)
    }
}

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

    let mut socket = ChatServer::bind(8000u16).await?;
    socket.run().await?;
    socket.close().await?;

    Ok(())
}
