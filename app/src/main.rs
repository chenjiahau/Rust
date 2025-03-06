mod utils;

use std::collections::HashMap;

use actix_web::{get, middleware::Logger, App, HttpServer, Responder};
use httpstatus::StatusCode;

use utils::api_response::{
    response,
    response_with_data,
    response_error,
    response_not_found,
    response_bad_request
};

#[get("/greet")]
async fn greet() -> impl Responder {
    response(StatusCode::Ok, "Hello, World!")
}

#[get("/version")]
async fn version() -> impl Responder {
    let mut data = HashMap::new();
    data.insert("app", "Rust Actix Web");
    data.insert("version", "1.0.0");

    // response_with_data(StatusCode::Ok, "Success", Some({}))
    response_with_data(StatusCode::Ok, "Success", Some(data))
}

#[get("/error")]
async fn error() -> impl Responder {
    response_error(StatusCode::InternalServerError, "Internal Server Error")
}

#[get("/notfound")]
async fn not_found() -> impl Responder {
    response_not_found(StatusCode::NotFound, "Not Found")
}

#[get("/badrequest")]
async fn bad_request() -> impl Responder {
    response_bad_request(StatusCode::BadRequest, "Bad Request")
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
            .service(error)
            .service(not_found)
            .service(bad_request)
    })
    .bind((address, port))?
    .run()
    .await
}