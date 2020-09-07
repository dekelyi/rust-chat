use anyhow::{self, Context};
use std::net::UdpSocket;

fn main() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").context("failed to bind adress")?;
    println!("listening on {}", socket.local_addr()?);
    Ok(())
}
