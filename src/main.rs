use redis_clone::server::RedisServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = RedisServer::new("0.0.0.0:8888");
    server.run().await
}
