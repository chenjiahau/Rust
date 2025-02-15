mod utils;

use std::collections::HashMap;

use actix_web::{get, middleware::Logger, App, HttpServer, Responder};
use utils::api_response::{HttpState, response, response_with_data};

#[get("/greet")]
async fn greet() -> impl Responder {
    response(HttpState::Ok, "Hello, World!")
}

#[get("/version")]
async fn version() -> impl Responder {
    let mut data = HashMap::new();
    data.insert("app", "Rust Actix Web");
    data.insert("version", "1.0.0");

    // response_with_data(HttpState::Ok, "Success", Some({}))
    response_with_data(HttpState::Ok, "Success", Some(data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Initialize the logger
    env_logger::init();

    // Load the environment variables
    dotenv::dotenv().ok();
    let address = (utils::constants::ADDRESS).clone();
    let port: u16 = (utils::constants::PORT).clone();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(greet)
            .service(version)
    })
    .bind((address, port))?
    .run()
    .await
}