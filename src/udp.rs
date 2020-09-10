use anyhow::{self, Context};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net, sync::Mutex, task};

/// a chat server instance
pub struct UdpServer {
    listener: Arc<Mutex<net::UdpSocket>>,
}

impl UdpServer {
    /// handle a new connection
    async fn handle_msg(buf: Vec<u8>, addr: SocketAddr) -> Vec<u8> {
        log::info!("{}: Got packet", addr);
        buf
    }

    /// run the main event loop
    pub async fn run(&mut self) {
        loop {
            let mut buf = [0u8; 32];
            match self.listener.lock().await.recv_from(&mut buf).await {
                Err(_) => continue,
                Ok((_, addr)) => {
                    let socket = self.listener.clone();
                    let _res = task::spawn(async move {
                        let res = UdpServer::handle_msg(buf.to_vec(), addr).await;
                        socket.lock().await.send_to(&res[..], addr).await.unwrap();
                    });
                }
            }
        }
    }

    /// Create a new instance of a bound server
    pub async fn bind(port: u16) -> anyhow::Result<Self> {
        let listener = net::UdpSocket::bind(("127.0.0.1", port))
            .await
            .context("failed to bind adress")?;
        let listener = Arc::new(Mutex::new(listener));
        let res = Self { listener };
        log::info!("listening on {}", res.listener.lock().await.local_addr()?);
        Ok(res)
    }
}
