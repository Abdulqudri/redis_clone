use super::connection::Connection;
use std::io;
use tokio::net::TcpListener;
pub struct RedisServer {
    address: String,
}

impl RedisServer {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub async fn run(&self) -> io::Result<()> {
        let listener: TcpListener = TcpListener::bind(&self.address).await?;
        println!("Redis server listening on {}", self.address);
        loop {
            let (stream, addr) = listener.accept().await?;
            println!("Accepted connection from {}", addr);
            tokio::spawn(async move {
                if let Err(e) = Connection::new(stream).process().await {
                    eprintln!("Connection error: {}", e)
                }
            });
        }
    }
}
