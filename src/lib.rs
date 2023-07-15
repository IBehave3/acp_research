use actix_web::{web, App, HttpServer};
use startup::on_startup;
use startup::API_CONFIG;

mod controller;
mod infra;
mod startup;

pub async fn start_server() -> std::io::Result<()> {
    on_startup().await;

    let host = &API_CONFIG.get().unwrap().host;
    let port = API_CONFIG.get().unwrap().port;

    println!("Info server listening at {host}:{port}");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                // NOTE: test endpoints
                .service(controller::test::test_get_handler)
                // NOTE: push_data endpoints
                .service(controller::push_data::push_data_get_handler)
                .service(controller::push_data::push_data_post_handler)
                // NOTE: notification endpoints
                .service(controller::notification::notification_get_handler)
                .service(controller::notification::notification_post_handler)
                // NOTE: device endpoints
                .service(controller::device::device_post_handler)
                .service(controller::device::reset_device_get_handler)
                .service(
                    web::scope("/auth")
                        // NOTE: auth endpoints
                        .service(controller::auth::create_user_post_handler)
                        .service(controller::auth::login_user_get_handler)
                        .service(controller::auth::remove_user_delete_handler),
                )
                .service(
                    web::scope("/manage")
                        // NOTE: container endpoints
                        .service(controller::manage::container_get_handler)
                        .service(controller::manage::container_name_get_handler),
                ),
        )
    })
    .bind((host.clone(), port))?
    .run()
    .await
}
