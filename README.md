# acp_research 
- acp_research is an API built with Rust and MongoDb to store and analyze data.

## Installation
- Tested using Ubuntu LTS 22 
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

## Configurint API as Service
```
sudo cp /home/ubuntu/acp_research.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl start acp_research.service
sudo systemctl status acp_research.service
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
sudo cp nginx/nginx.conf /etc/nginx/nginx.conf
sudo cp -r cert/ /etc/nginx/
sudo systemctl restart nginx
```

## Connection to Test Server
```
ssh -i "AcpResearch.pem" ubuntu@ec2-18-207-248-247.compute-1.amazonaws.com
```

## Setup on Test Server
```
sudo apt install git gh gcc
gh auth login
gh repo clone https://github.com/IBehave3/acp_research
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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
sudo ./target/release/acp_research >> output.log 2>&1
```

## Login db admin
```
mongosh admin -u {admin_username} -p {admin_password}
```

## Setup db auth
```
mongosh
use admin
db.createUser(
  {
    user: "admin",
    pwd: "{admin_password}",
    roles: [ { role: "userAdminAnyDatabase", db: "admin" } ]
  }
)
use {db_name}

db.createUser(
  {
    user: "internal_user",
    pwd: "{internal_user_password}",
    roles: [ { role: "readWrite", db: "acp_research_dev_db" } ]
  }
)

db.createUser(
  {
    user: "external_user",
    pwd: "{external_user_password}",
    roles: [ { role: "read", db: "acp_research_dev_db" } ]
  }
)
```
- Setup authentaction in /etc/mongodb.conf
```
security:
  authorization: enabled
net:
  port: 27017
  bindIp: 0.0.0.0
```