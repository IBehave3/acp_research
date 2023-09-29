use core::panic;



use actix_web::middleware::Logger;
use actix_web::{web, web::Data, App, HttpServer};

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use env_logger::Env;
use log::{info, error};

use api::startup::on_api_startup;
use api::startup::API_CONFIG;

use polling::startup::on_polling_startup;
use share::model::DATABASE_CONFIG;
use tokio::task;
use tokio::time::{sleep, Duration};

use crate::polling::poller::airthings_poller::airthings_poll;
use crate::polling::poller::gray_wolf_poller::gray_wolf_poll;
use crate::polling::poller::keychain_poller::keychain_poll;
use crate::polling::poller::uhoo_business_poller::uhoo_business_poll;
use crate::polling::poller::uhoo_home_poller::uhoo_home_poll;
use crate::api::infra::jwt_middleware;
use crate::api::model::jwt_model::JwtToken;

mod api;
mod schema;
mod polling;
mod share;

const POLL_INTERVAL_IN_SECONDS: u64 = 120;

pub async fn start_polling() -> std::io::Result<()> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info"))
        .expect("Error failed to init logger");

    info!("starting acp_research_polling");

    on_polling_startup().await;

    let database_config = match DATABASE_CONFIG.get() {
        Some(database_config) => database_config,
        None => panic!("Error DATABASE_CONFIG not initialized"),
    };

    info!("\n{:#?}", database_config);

    loop {
        if let Err(err) =  tokio::try_join!(
            task::spawn(keychain_poll()),
            task::spawn(airthings_poll()),
            task::spawn(uhoo_business_poll()),
            task::spawn(uhoo_home_poll()),
            task::spawn(gray_wolf_poll()),
        ) {
            error!("{err}");
        }

        sleep(Duration::from_secs(POLL_INTERVAL_IN_SECONDS)).await;
    }
}

pub async fn start_server() -> std::io::Result<()> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info"))
        .expect("Error failed to init logger");

    info!("starting acp_research_api");

    JwtToken::init_jwt_key();

    on_api_startup().await;

    let api_config = match API_CONFIG.get() {
        Some(api_config) => api_config,
        None => panic!("Error API_CONFIG not initialized"),
    };
    let database_config = match DATABASE_CONFIG.get() {
        Some(database_config) => database_config,
        None => panic!("Error DATABASE_CONFIG not initialized"),
    };

    info!("\n{:#?}", api_config);
    info!("\n{:#?}", database_config);

    let host = &api_config.host;
    let port = api_config.port;
    let database_connection_string = &database_config.database_url;

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_connection_string);
    let pool = match Pool::builder(config).max_size(25).build() {
        Ok(pool) => pool,
        Err(err) => {
            panic!("{err}");
        }
    };

    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(api::presentation::test_presentation::test_get_handler)
                    .service(
                        web::scope("/auth-init")
                            .service(api::presentation::user_presentation::create_user_post_handler)
                            .service(api::presentation::user_presentation::login_user_get_handler),
                    )
                    .service(
                        web::scope("/auth")
                            .wrap(jwt_middleware::Auth)
                            .service(api::presentation::user_presentation::information_user_get_handler)
                            .service(api::presentation::user_presentation::airthings_user_patch_handler)
                            .service(api::presentation::user_presentation::gray_wolf_user_patch_handler)
                            .service(api::presentation::user_presentation::uhoo_business_user_patch_handler)
                            .service(api::presentation::user_presentation::uhoo_home_user_patch_handler)
                            .service(api::presentation::user_presentation::keychain_user_patch_handler)
                    )
                    .service(
                        web::scope("/fitbit")
                            .wrap(jwt_middleware::Auth)
                            .service(api::presentation::fitbit_presentation::create_fitbit_post_handler),
                    )
                    .service(
                        web::scope("/survey")
                            .wrap(jwt_middleware::Auth)
                            .service(api::presentation::survey_presentation::create_hourly_survey_post_presentation)
                            .service(api::presentation::survey_presentation::create_daily_survey_post_presentation),
                    )
                    .service(
                        web::scope("/location")
                            .wrap(jwt_middleware::Auth)
                            .service(api::presentation::gis_location_presentation::create_gis_location_post_presentation)
                            .service(api::presentation::user_location_presentation::create_user_location_post_presentation)
                    )
                    .service(
                        web::scope("/vehicle-measurement")
                            .wrap(jwt_middleware::Auth)
                            .service(api::presentation::vehicle_presentation::create_vehicle_measurement_post_handler)
                    ),
            )
    })
    .bind((host.clone(), port))?
    .run();

    info!("api server listening at {host}:{port}");
    api_server.await?;

    Ok(())
}
