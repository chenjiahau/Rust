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
    pub static ref SMTP_HOST: String = set_smtp_host();
    pub static ref SMTP_PORT: u16 = set_smtp_port();
    pub static ref SMTP_USERNAME: String = set_smtp_username();
    pub static ref SMTP_PASSWORD: String = set_smtp_password();
    pub static ref SMTP_SENDER: String = set_smtp_sender();
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

fn set_smtp_host() -> String {
    env::var("SMTP_HOST").unwrap().to_string()
}

fn set_smtp_port() -> u16 {
    env::var("SMTP_PORT").unwrap().parse().unwrap()
}

fn set_smtp_username() -> String {
    env::var("SMTP_USERNAME").unwrap().to_string()
}

fn set_smtp_password() -> String {
    env::var("SMTP_PASSWORD").unwrap().to_string()
}

fn set_smtp_sender() -> String {
    env::var("SMTP_SENDER").unwrap().to_string()
}