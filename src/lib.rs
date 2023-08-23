
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};


use env_logger::Env;
use log::info;
use startup::on_startup;
use startup::API_CONFIG;


use crate::infra::airthings_integ::start_airthings_poll;
use crate::infra::gray_wolf_integ::start_gray_wolf_poll;
use crate::infra::uhoo_aura_integ::start_uhoo_aura_poll;

mod controller;
mod infra;
mod model;
mod presentation;
mod startup;

pub async fn start_server() -> std::io::Result<()> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info"))
        .expect("Error failed to init logger");

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
    let socket_port = api_config.socketport;

    info!("server listening at {host}:{port}");
    info!("socket server listening at {host}:{socket_port}");

    // NOTE: web socket server
    HttpServer::new(|| App::new().route("/socket", web::get().to(presentation::socket::socket_handler)))
    .bind(("127.0.0.1", api_config.socketport))?
    .run()
    .await?;

    // NOTE: http server
    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(
            web::scope("/api")
                // NOTE: test endpoints
                .service(presentation::test::test_get_handler)
                // NOTE: push_data endpoints
                .service(presentation::push_data::push_data_get_handler)
                .service(presentation::push_data::push_data_post_handler)
                // NOTE: notification endpoints
                .service(presentation::notification::notification_get_handler)
                .service(presentation::notification::notification_post_handler)
                // NOTE: datastructure endpoints
                .service(presentation::datastructure::datastructure_post_handler)
                .service(presentation::datastructure::reset_datastructure_get_handler)
                // NOTE: device endpoints
                .service(presentation::device::device_delete_handler)
                .service(
                    web::scope("/auth")
                        // NOTE: auth endpoints
                        .service(presentation::auth::create_user_post_handler)
                        .service(presentation::auth::login_user_get_handler)
                        .service(presentation::auth::remove_user_delete_handler),
                )
                .service(
                    web::scope("/manage")
                        // NOTE: container endpoints
                        .service(presentation::manage::container_get_handler)
                        .service(presentation::manage::container_name_get_handler),
                ),
        )
    })
    .bind((host.clone(), port))?
    .run()
    .await?;

    Ok(())
}
