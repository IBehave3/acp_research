use std::sync::OnceLock;


use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;

pub static CONNECTION_POOL: OnceLock<Pool<AsyncPgConnection>> = OnceLock::new();