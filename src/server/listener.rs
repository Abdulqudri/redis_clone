use std::io::Error;

use tokio::net::TcpListener;
use crate::server::Connection;

pub struct RedisServer {
    addr: String,
}

impl RedisServer {
    pub fn new(addr: String) -> Self {
        RedisServer { addr }
    }

    pub async fn start(&self) -> Result<(), Error>{
        // Logic to start the Redis server
        println!("Starting Redis server at {}", self.addr); 
        let listener = TcpListener::bind(&self.addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;
            let mut connection = Connection::new(stream);
            println!("New client connected: {}", self.addr);
            tokio::spawn(async move {
                connection.handle().await;
            });
        }
    }

    pub fn addr(&self) -> &str {
        &self.addr
    }
}
