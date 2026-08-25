use std::io::{ErrorKind, Result};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::command::{Command, CommandHandler};
use crate::protocol::{RespParser, RespSerializer};

pub struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    parser: RespParser,
    serializer: RespSerializer,
    handler: CommandHandler,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream,
            buffer: Vec::with_capacity(1024),
            parser: RespParser::new(),
            serializer: RespSerializer::new(),
            handler: CommandHandler::new(),
        }
    }

    pub async fn handle(&mut self) -> Result<()> {
        // Logic to handle the connection
        let mut buffer = [0; 1024];
        loop {
            match self.stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("Client disconnected");
                    return Ok(());
                }
                Ok(n) => {
                    println!("Received {} bytes", n);
                    self.buffer.extend_from_slice(&buffer[..n]);
                    loop {
                        if self.buffer.is_empty() {
                            break;
                        }
                        match self.parser.parse(&self.buffer) {
                            Ok((value, consumed)) => {
                                let out_resp = match Command::from_resp(value) {
                                    Ok(command) => self.handler.execute(command),
                                    Err(err_resp) => err_resp,
                                };

                                let serialized_bytes = self.serializer.serialize(&out_resp);

                                self.stream.write_all(&serialized_bytes).await?;
                                self.buffer.drain(..consumed);
                            }

                            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                                // Incomplete command, wait for more data
                                break;
                            }

                            Err(e) => {
                                eprintln!("Protocol parsing error encountered: {:?}", e);
                                let error_frame = b"-ERR Protocol error: invalid RESP\r\n";
                                let _ = self.stream.write_all(error_frame).await?;
                                return Err(e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    return Err(e);
                }
            }
        }
    }
}
