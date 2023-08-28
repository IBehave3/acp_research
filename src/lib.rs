use core::panic;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use env_logger::Env;
use log::info;
use startup::on_startup;
use startup::API_CONFIG;

use crate::controller::jwt;
use crate::infra::airthings_integ::start_airthings_poll;
use crate::infra::gray_wolf_integ::start_gray_wolf_poll;
use crate::infra::jwt_middleware;
use crate::infra::uhoo_aura_integ::start_uhoo_aura_poll;
use crate::model::jwt::JwtToken;

mod controller;
mod infra;
mod model;
mod presentation;
mod startup;

pub async fn start_server() -> std::io::Result<()> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info"))
        .expect("Error failed to init logger");

    JwtToken::init_jwt_key();

    on_startup().await;

    let api_config = match API_CONFIG.get() {
        Some(api_config) => api_config,
        None => panic!("Error API_CONFIG not initialized"),
    };

    // NOTE: start polling
    if api_config.pollsensors {
        start_airthings_poll();
        start_uhoo_aura_poll();
        start_gray_wolf_poll();
    }

    let host = &api_config.host;
    let port = api_config.port;

    let api_server = HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(
            web::scope("/api")
                .service(presentation::test::test_get_handler)
                .service(
                    web::scope("/auth-init")
                        .service(presentation::auth::create_user_post_handler)
                        .service(presentation::auth::login_user_get_handler),
                )
                .service(
                    web::scope("/auth")
                        .wrap(jwt_middleware::Auth)
                        .service(presentation::auth::information_user_get_handler)
                        .service(presentation::auth::airthings_user_patch_handler)
                        .service(presentation::auth::gray_wolf_user_patch_handler)
                        .service(presentation::auth::uhoo_aura_user_patch_handler)
                ),
        )
    })
    .bind((host.clone(), port))?
    .run();

    info!("api server listening at {host}:{port}");
    api_server.await?;

    Ok(())
}
