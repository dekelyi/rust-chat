use anyhow::{self, Context};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net, sync::RwLock, task};

/// a chat server instance
pub struct UdpServer {
    listener: Arc<RwLock<net::UdpSocket>>,
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
            match self.listener.write().await.recv_from(&mut buf).await {
                Err(_) => continue,
                Ok((_, addr)) => {
                    let socket = self.listener.clone();
                    let _res = task::spawn(async move {
                        let res = UdpServer::handle_msg(buf.to_vec(), addr).await;
                        socket.write().await.send_to(&res[..], addr).await.unwrap();
                    });
                }
            }
        }

        // while let Some(stream) = self.listener.incoming().next().await {
        //     let mut stream = match stream {
        //         Ok(s) => s,
        //         Err(err) => {
        //             log::error!("{}", err);
        //             continue;
        //         }
        //     };
        //     let _handle = task::spawn(async move {
        //         UdpServer::handle_connections(&mut stream).await;
        //     });
        // }
    }

    /// Create a new instance of a bound server
    pub async fn bind(port: u16) -> anyhow::Result<Self> {
        let listener = net::UdpSocket::bind(("127.0.0.1", port))
            .await
            .context("failed to bind adress")?;
        let listener = Arc::new(RwLock::new(listener));
        let res = Self { listener };
        log::info!("listening on {}", res.listener.read().await.local_addr()?);
        Ok(res)
    }
}
