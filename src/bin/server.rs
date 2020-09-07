use anyhow::{self, Context};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:0")
        .await
        .context("failed to bind adress")?;
    println!("listening on {}", socket.local_addr()?);
    Ok(())
}
