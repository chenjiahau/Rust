use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NAME: String = set_name();
    pub static ref VERSION: String = set_version();
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
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