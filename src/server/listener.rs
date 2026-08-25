use std::io::Error;

use crate::server::Connection;
use tokio::net::TcpListener;

pub struct RedisServer {
    addr: String,
}

impl RedisServer {
    pub fn new(addr: String) -> Self {
        RedisServer { addr }
    }

    pub async fn start(&self) -> Result<(), Error> {
        // Logic to start the Redis server
        println!("Starting Redis server at {}", self.addr);
        let listener = TcpListener::bind(&self.addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;
            let mut connection = Connection::new(stream);
            println!("New client connected: {}", self.addr);
            tokio::spawn(async move {
                connection
                    .handle()
                    .await
                    .unwrap_or_else(|e| eprintln!("Error handling connection: {}", e));
            });
        }
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }
}
