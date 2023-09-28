use acp_research::start_polling;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    start_polling().await
}
