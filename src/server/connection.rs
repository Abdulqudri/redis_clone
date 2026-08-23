use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub async fn handle(&mut self) {
        // Logic to handle the connection
        loop {
            let mut buffer = [0; 1024];
            match self.stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(n) => {
                    println!("Received {} bytes", n);
                    let received_bytes = &buffer[..n];
                    let received_text = String::from_utf8_lossy(received_bytes);
                    println!("Received data: {}", received_text);
                    if let Err(e) = self.stream.write_all(received_bytes).await {
                        eprintln!("Failed to write to socket; error = {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        }
    }
}
