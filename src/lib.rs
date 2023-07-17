use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use startup::on_startup;
use startup::API_CONFIG;

mod controller;
mod infra;
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

    let host = &api_config.host;
    let port = api_config.port;

    println!("Info server listening at {host}:{port}");

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
                // NOTE: device endpoints
                .service(presentation::datastructure::datastructure_post_handler)
                .service(presentation::datastructure::reset_datastructure_get_handler)
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
    .await
}
