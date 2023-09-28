echo "building api"
cargo build --release --bin acp_research_api
echo "building polling"
cargo build --release --bin acp_research_polling

echo "copying api"
sudo cp ./target/release/acp_research_api ./prod/acp_research_api
echo "copying polling"
sudo cp ./target/release/acp_research_polling ./prod/acp_research_polling