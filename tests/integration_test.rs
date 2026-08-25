use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use redis_clone::server::Connection; // Ensure server modules are declared public in lib.rs

#[tokio::test]
async fn test_end_to_end_pipeline() {
    // 1. Bind to an ephemeral loopback address
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // 2. Spawn the connection handler loop background service
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut connection = Connection::new(stream);
        let _ = connection.handle().await;
    });

    // 3. Connect client stream
    let mut client = TcpStream::connect(addr).await.unwrap();
    let mut read_buffer = vec![0; 1024];

    // Case A: PING pipeline validation
    client.write_all(b"*1\r\n$4\r\nPING\r\n").await.unwrap();
    let n = client.read(&mut read_buffer).await.unwrap();
    assert_eq!(&read_buffer[..n], b"+PONG\r\n");

    // Case B: ECHO pipeline validation
    client.write_all(b"*2\r\n$4\r\nECHO\r\n$5\r\nhello\r\n").await.unwrap();
    let n = client.read(&mut read_buffer).await.unwrap();
    assert_eq!(&read_buffer[..n], b"$5\r\nhello\r\n");
}
