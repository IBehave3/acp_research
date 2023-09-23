echo "building prod exe"
cargo build --release
echo "copying prod exe to ./prod/acp_research"
sudo cp ./target/release/acp_research ./prod/acp_research
echo "copying .prod.env to ./prod/.env"
sudo cp ./.prod.env ./prod/.env