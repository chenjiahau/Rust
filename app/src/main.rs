mod utils;
mod handlers;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};

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

    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::basic_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}