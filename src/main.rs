use redis_clone::server::RedisServer;

#[tokio::main]
async fn main() {
    let server = RedisServer::new("127.0.0.1:6379".into());
    match server.start().await{
        Ok(_) => println!("Server started successfully"),
        Err(e) => eprintln!("Error starting server: {}", e),
    }
}
