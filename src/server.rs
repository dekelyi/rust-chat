use crate::packet;
use anyhow::{self, Context};
use std::convert::{TryFrom, TryInto};
use std::net::SocketAddr;
use tokio::{net, sync::mpsc, task};

/// a chat server instance
pub struct ChatServer {
    listener: net::UdpSocket,
}

impl ChatServer {
    /// handle a new connection
    async fn handle_msg(buf: Vec<u8>, addr: SocketAddr) -> anyhow::Result<Vec<u8>> {
        log::info!("{}: Got packet", addr);
        let buf: &[u8] = &buf;
        let msg = packet::Packet::try_from(buf)?;
        msg.try_into()
    }

    /// run the main event loop
    pub async fn run(&mut self) {
        let (tx, mut rx) = mpsc::unbounded_channel::<(Vec<u8>, SocketAddr)>();
        let mut buf = [0u8; 32];
        loop {
            tokio::select! {
                Some((res, addr)) = rx.recv() => {
                    self.listener.send_to(&res, addr).await.unwrap();
                },
                Ok((n, addr)) = self.listener.recv_from(&mut buf) => {
                    if n == 0 { continue };
                    let tx = tx.clone();
                    task::spawn(async move {
                        let res = ChatServer::handle_msg(buf.to_vec(), addr).await;
                        let res = match res {
                            Ok(buf) => buf,
                            Err(err) => {
                                let msg = packet::Packet::Error { err: err.to_string() };
                                msg.try_into().expect("failed to parse an error")
                            }
                        };
                        tx.send((res, addr)).unwrap();
                    });
                },
            };
        }
    }

    /// Create a new instance of a bound server
    pub async fn bind(port: u16) -> anyhow::Result<Self> {
        let listener = net::UdpSocket::bind(("127.0.0.1", port))
            .await
            .context("failed to bind adress")?;
        // let listener = Arc::new(Mutex::new(listener));
        let res = Self { listener };
        log::info!("listening on {}", res.listener.local_addr()?);
        Ok(res)
    }
}
