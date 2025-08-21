use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NAME: String = set_name();
    pub static ref VERSION: String = set_version();
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref TOKEN_EXPIRATION_TIME: i64 = set_token_expiration_time();
    pub static ref SECRET_KEY: String = set_secret_key();
    pub static ref STATIC_PATH: String = set_static_path();
}

fn set_name() -> String {
    env::var("APP_NAME").unwrap().to_string()
}

fn set_version() -> String {
    env::var("APP_VERSION").unwrap().to_string()
}

fn set_address() -> String {
    env::var("ADDRESS").unwrap().to_string()
}

fn set_port() -> u16 {
    env::var("PORT").unwrap().parse().unwrap()
}

fn set_database_url() -> String {
    env::var("DATABASE_URL").unwrap().to_string()
}

fn set_token_expiration_time() -> i64 {
    env::var("TOKEN_EXPIRATION_TIME")
        .unwrap()
        .parse()
        .unwrap()
}

fn set_secret_key() -> String {
    env::var("SECRET_KEY").unwrap().to_string()
}

fn set_static_path() -> String {
    env::var("STATIC_PATH").unwrap().to_string()
}