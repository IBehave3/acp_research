# acp_research 
- acp_research is an API built with Rust and MongoDb to store and analyze data.

## Installation
- Tested using Ubuntu LTS 22.04 
- Use the rust book guide https://doc.rust-lang.org/book/ch01-01-installation.html#installation

## Configuring API
```
cp default.env .env
```

## Building and Running API
```bash
cargo build --release
cargo run --release
```

## Connection to Test Server
```
ssh -i "AcpResearch.pem" ubuntu@ec2-18-207-248-247.compute-1.amazonaws.com
```

## Install Docker
- https://docs.docker.com/engine/install

## Running DB for Local Development
```
cd {project_dir}
cp default.env .env
docker compose -f docker-compose-dev.yml up -d
```

## Installing Diesel for Local Development
- https://diesel.rs/guides/getting-started

## Fitbit Sense 2 App manager
- https://dev.fitbit.com/

## All Day Data Collection Fitbit
- https://www.fitabase.com/resources/knowledge-base/getting-started/syncing-fitbit-devices/