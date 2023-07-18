# acp_research 
- acp_research is an API built with Rust and MongoDb to store and analyze data.

## Installation
- Tested using an Ubuntu LTS 22 
- Use the rust book guide https://doc.rust-lang.org/book/ch01-01-installation.html#installation
- Use the mongodb site to https://www.mongodb.com/docs/manual/tutorial/install-mongodb-on-ubuntu

## Extra Tools
- MongoDb Gui utility https://www.mongodb.com/try/download/compass 

## Managing Mongodb 
```bash
sudo systemctl enable mongod
sudo systemctl stop mongod
sudo systemctl restart mongod

service mongod start
service monogd stop
service mongod status
```

## Configuring API
```
cp default.env .env
```

## Building and Running API
```bash
cargo build --release
cargo run --release
```

## Setting up Nginx
```
sudo apt update
sudo apt install nginx
sudo ufw app list
sudo ufw allow 'Nginx HTTP'
sudo ufw allow 'Nginx HTTPS'
sudo ufw enable
sudo cp nginx/nginx.confg /etc/nginx/nginx.conf
sudo cp -r cert/ /etc/nginx/
```

## Connection to Test Server
```
ssh -i "AcpResearch.pem" ubuntu@ec2-18-207-248-247.compute-1.amazonaws.com
```

## Setup on Test Server
```
sudo apt install git
sudo apt install gh
sudo apt install gcc
gh auth login
gh repo clone https://github.com/IBehave3/acp_research
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt-get install gnupg curl
curl -fsSL https://pgp.mongodb.com/server-6.0.asc | \
   sudo gpg -o /usr/share/keyrings/mongodb-server-6.0.gpg \
   --dearmor
echo "deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb-server-6.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/6.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list
sudo apt-get update
sudo apt-get install -y mongodb-org
sudo systemctl start mongod
sudo systemctl status mongod
mongosh
use acp_research_dev_db
cargo run --release
```
