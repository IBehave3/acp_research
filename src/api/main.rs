use acp_research::start_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}
