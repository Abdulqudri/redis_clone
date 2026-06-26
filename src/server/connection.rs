use std::io::Result;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub async fn process(mut self) -> Result<()> {
        println!("incoming connection from: {}", self.stream.peer_addr()?);
        let mut buf = [0; 4096];
        loop {
            let byte_read = self.stream.read(&mut buf).await?;
            if byte_read == 0 {
                println!("Client disconnected: {}", self.stream.peer_addr()?);
                return Ok(());
            }
            self.stream.write_all(&buf[..byte_read]).await?;
        }
    }
}
