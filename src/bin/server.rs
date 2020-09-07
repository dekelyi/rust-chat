use anyhow::{self, Context};
use std::net::TcpListener;

fn main() -> anyhow::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:0").context("failed to bind adress")?;
    println!("listening on {}", socket.local_addr()?);
    Ok(())
}
