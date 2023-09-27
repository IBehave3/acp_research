use chrono::Datelike;
use core::panic;
use log::{error, info};
use reqwest::{Client, Response};

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use urlencoding::encode;

use crate::controller::{user_controller, keychain_controller};
use crate::model::keychain_model::ClientKeychain;
use crate::model::user_model::UserKeychain;

use super::database::DbPool;


const QUERY_FREQ_SECS: u64 = 60;

pub async fn get_device_data(
    api_key: &str,
    dev_mac: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    let current_date = chrono::Utc::now();
    let url = format!("https://api.atmotube.com/api/v1/data?api_key={}&mac={}&start_date={}-{}-{}", 
    api_key, encode(dev_mac), 
    current_date.year(), 
    current_date.month(), 
    current_date.day());

    let response = Client::new()
        .get(url)
        .send()
        .await?;

    Ok(response)
}

pub fn start_keychain_poll(pool: Arc<DbPool>) {
    thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(err) => {
                error!("{err}");
                panic!("unable to start keychain poll");
            }
        };

        loop {
            let connection = &mut rt.block_on(pool.get()).unwrap();

            let user_keychains: Vec<UserKeychain> = match rt.block_on(user_controller::get_keychain_users(connection)) {
                Ok(users) => users,
                Err(err) => {
                    error!("{err}");
                    continue;
                }
            };

            for user_keychain in user_keychains {
                let device_macs = match user_keychain.devicemacs {
                    Some(device_macs) => device_macs,
                    None => {
                        continue;
                    }
                };

                let api_key = user_keychain.apikey;

                for device_mac in device_macs {
                    let device_mac = match device_mac {
                        Some(device_mac) => device_mac,
                        None => {
                            continue;
                        }
                    };
                    let response =
                        match rt.block_on(get_device_data(&api_key, &device_mac)) {
                            Ok(response) => response,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                    if response.status() == 200 {
                        let user_ref_id = user_keychain.userid;
                        info!("keychain writing data for (user_id, device_mac): ({user_ref_id}, {device_mac})");

                        let bytes = match rt.block_on(response.bytes()) {
                            Ok(bytes) => bytes,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };
                        let client_keychain: ClientKeychain = match serde_json::from_slice(&bytes[..]) {
                            Ok(data) => data,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            }
                        };

                        if let Err(err) = rt.block_on(keychain_controller::create_keychain(connection, client_keychain, user_ref_id, &device_mac)) {
                            error!("uhoo_business db error {err}");
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(QUERY_FREQ_SECS));
        }
    });
}
