mod routes;
mod utils;

use actix_web::{middleware::Logger, App, web, HttpServer};
use sea_orm::Database;
use migration::{Migrator, MigratorTrait};

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
    let database_url = (utils::constants::DATABASE_URL).clone();

    // Connect to the database
    let db = Database::connect(database_url).await.unwrap();
    // Run the migrations when the application starts
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(utils::app_state::AppState { db: db.clone() })) // Pass the database connection to the application state
            .wrap(Logger::default()) // Log the requests
            .configure(routes::greet_routes::config) // Configure greet routes
            .configure(routes::api_routes::config) // Configure API routes
    })
    .bind((address, port))? // bind the server to the address and port
    .run()
    .await
}