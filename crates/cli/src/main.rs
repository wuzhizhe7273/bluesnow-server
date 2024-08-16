#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config=server::ServerConfig::new();
    let server = server::Server::new(server_config);
    server.run().await?;
    Ok(())
}
