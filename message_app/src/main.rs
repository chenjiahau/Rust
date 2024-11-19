mod routes;
mod utils;

use actix_web::{middleware::Logger, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set the environment variable `RUST_LOG` to `actix_web=info` to see logs
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
            .wrap(Logger::default()) // Log the requests
            .configure(routes::greet_route::config) // Configure the routes
    })
    .bind((address, port))? // bind the server to the address and port
    .run()
    .await
}