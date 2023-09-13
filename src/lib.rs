use core::panic;
use std::sync::Arc;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, web::Data};

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use env_logger::Env;
use log::info;
use startup::on_startup;
use startup::API_CONFIG;

use crate::infra::airthings_integ::start_airthings_poll;
use crate::infra::gray_wolf_integ::start_gray_wolf_poll;
use crate::infra::jwt_middleware;
use crate::infra::uhoo_aura_integ::start_uhoo_aura_poll;
use crate::model::jwt_model::JwtToken;
use crate::startup::DATABASE_CONFIG;

mod controller;
mod infra;
mod model;
mod presentation;
mod startup;
mod schema;

pub async fn start_server() -> std::io::Result<()> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info"))
        .expect("Error failed to init logger");

    JwtToken::init_jwt_key();

    on_startup().await;

    let api_config = match API_CONFIG.get() {
        Some(api_config) => api_config,
        None => panic!("Error API_CONFIG not initialized"),
    };
    let database_config = match DATABASE_CONFIG.get() {
        Some(database_config) => database_config,
        None => panic!("Error DATABASE_CONFIG not initialized"),
    };

    let host = &api_config.host;
    let port = api_config.port;
    let database_connection_string = &database_config.database_url;

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_connection_string);
    let pool = match Pool::builder(config).max_size(15).build() {
        Ok(pool) => pool,
        Err(err) => {
            panic!("{err}");
        }
    };

    let pool_arc = Arc::new(pool);

    // NOTE: start polling
    if api_config.pollsensors {
        start_airthings_poll(pool_arc.clone());
        start_uhoo_aura_poll(pool_arc.clone());
        start_gray_wolf_poll(pool_arc.clone());
    }

    let app_data = Data::from(pool_arc);

    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(presentation::test_presentation::test_get_handler)
                    .service(
                        web::scope("/auth-init")
                            .service(presentation::user_presentation::create_user_post_handler)
                            .service(presentation::user_presentation::login_user_get_handler),
                    )
                    .service(
                        web::scope("/auth")
                            .wrap(jwt_middleware::Auth)
                            .service(presentation::user_presentation::information_user_get_handler)
                            .service(presentation::user_presentation::airthings_user_patch_handler)
                            .service(presentation::user_presentation::gray_wolf_user_patch_handler)
                            .service(presentation::user_presentation::uhoo_aura_user_patch_handler)
                    )
                    .service(
                        web::scope("/fitbit")
                            .wrap(jwt_middleware::Auth)
                            .service(presentation::fitbit_presentation::create_fitbit_post_handler),
                    )
                    .service(
                        web::scope("/survey")
                            .wrap(jwt_middleware::Auth)
                            .service(presentation::survey_presentation::create_hourly_survey_post_presentation)
                            .service(presentation::survey_presentation::create_daily_survey_post_presentation),
                    ),
            )
    })
    .bind((host.clone(), port))?
    .run();

    info!("api server listening at {host}:{port}");
    api_server.await?;

    Ok(())
}
