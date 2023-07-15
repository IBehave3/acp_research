// FIXME: put instructions on creating db in mongodb

# acp_research 
acp_research is an API built with Rust and MongoDb to store and analyze data.

## Installation
Tested is using an LTS Ubuntu OS
Use the rust book guide https://doc.rust-lang.org/book/ch01-01-installation.html#installation
Use the mongodb site to https://www.mongodb.com/docs/manual/tutorial/install-mongodb-on-ubuntu

## Extra Tools
MongoDb Gui utility https://www.mongodb.com/try/download/compass 

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
set correct values in .env

## Building and Running API
```bash
cargo build
cargo run
```
