use acp_research::start_server;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}
