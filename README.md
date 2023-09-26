# acp_research 
- acp_research is an API built with Rust and MongoDb to store and analyze data.

## Installation
- Tested using Ubuntu LTS 22 
- Use the rust book guide https://doc.rust-lang.org/book/ch01-01-installation.html#installation
- Use the postgres site to https://ubuntu.com/server/docs/databases-postgresql

## Managing PostgrsQL Database 
```

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

## Setup on Rust
```
sudo apt install git gh gcc libssl-dev
gh auth login
gh repo clone https://github.com/IBehave3/acp_research
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Setup PostgresQL
```
sudo apt install postgresql postgresql-client
sudo -u postgres psql
#login
CREATE DATABASE acp_research_db;
CREATE ROLE internal_user LOGIN PASSWORD 'password';
CREATE ROLE external_user LOGIN PASSWORD 'password';
ALTER USER internal_user SUPERUSER;
#logout
sudo psql -h localhost -d acp_research_db -U internal_user -p 27017
GRANT SELECT ON ALL TABLES IN SCHEMA public TO external_user;
#login
```

## Setup Diesel
```
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```

## Login db internal_user
```
psql -h localhost -U internal_user -d acp_research_db
```

## Allow outside connections in /etc/postgresql/*/main/postgresql.conf
```
listen_addresses = '*'
```
