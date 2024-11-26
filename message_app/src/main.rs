mod models;
mod routes;
mod utils;

use actix_web::{middleware::Logger, App, web, HttpServer};
use actix_cors::Cors;
use sea_orm::Database;
use migration::{Migrator, MigratorTrait};

#[derive(Debug)]
struct MainError {
    message: String
}

#[actix_web::main]
async fn main() -> Result<(), MainError> {
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
    let database_url = (utils::constants::DATABASE_URL).clone();

    // Connect to the database
    let db = Database::connect(database_url)
        .await
        .map_err(|err| MainError { message: err.to_string() })?;

    // Run the migrations when the application starts
    Migrator::up(&db, None)
        .await
        .map_err(|err| MainError { message: err.to_string() })?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(utils::app_state::AppState { db: db.clone() })) // Pass the database connection to the application state
            .wrap(Logger::default()) // Log the requests
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            ) // Enable CORS
            .configure(routes::greet_routes::config) // Configure greet routes
            .configure(routes::api_routes::config) // Configure API routes
    })
    .bind((address, port))
    .map_err(|err| MainError { message: err.to_string() })?
    .run()
    .await
    .map_err(|err| MainError { message: err.to_string() })
}