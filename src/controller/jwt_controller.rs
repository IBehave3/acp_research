use crate::model::jwt_model::{JwtCustomClaims, JwtToken};
use jwt_simple::prelude::*;
use log::info;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::OnceLock;

pub static JWT_KEY: OnceLock<HS256Key> = OnceLock::new();
const JWT_EXPIRATION_DURATION_IN_HOURS: u64 = 24;

impl JwtToken {
    pub fn init_jwt_key() {
        info!("checking for jwt private key");

        let jwt_private_key_file_path = Path::new(".jwt.private.key");
        let key: HS256Key;
        if jwt_private_key_file_path.exists() {
            info!("jwt private key found");
            let mut file = File::open(jwt_private_key_file_path.to_str().unwrap()).unwrap();
            let mut buffer = Vec::new();
            let file_size = file.read_to_end(&mut buffer).unwrap();

            if file_size <= 0 {
                panic!("jwt private key file size was {}", file_size);
            }

            key = HS256Key::from_bytes(&buffer);
        } else {
            info!("jwt private key not found");
            info!("creating new jwt private key and writing to .private.key");

            key = HS256Key::generate();

            let mut file = File::create(jwt_private_key_file_path.to_str().unwrap()).unwrap();
            file.write_all(&key.to_bytes()[..]).unwrap();
        }

        JWT_KEY.set(key).unwrap();
    }

    pub fn jwt_validate_token(self) -> Result<JWTClaims<JwtCustomClaims>, Box<dyn std::error::Error>> {
        let key = match JWT_KEY.get() {
            Some(key) => key,
            None => {
                panic!("jwt key not initialized");
            }
        };

        let claims = key.verify_token::<JwtCustomClaims>(&self.token, None)?;
        Ok(claims)
    }

    pub fn new(jwt_claims: JwtCustomClaims) -> Result<JwtToken, Box<dyn std::error::Error>> {
        let claims = Claims::with_custom_claims(
            jwt_claims,
            Duration::from_hours(JWT_EXPIRATION_DURATION_IN_HOURS),
        );

        let key = match JWT_KEY.get() {
            Some(key) => key,
            None => {
                panic!("jwt key not initialized");
            }
        };

        Ok(JwtToken {
            token: key.authenticate(claims).unwrap(),
        })
    }
}
