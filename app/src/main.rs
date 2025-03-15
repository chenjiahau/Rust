mod utils;
mod handlers;
mod routes;
mod models;

use actix_web::{web, middleware::Logger, App, HttpServer};

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

    // Initialize the database connection
    let db = utils::db_connection::establish_connection(utils::constants::DATABASE_URL.to_string()).await;

    // Create the application state
    let app_state = utils::app_state::AppState { db };

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .configure(routes::basic_routes::config)
            .configure(routes::unauth_routes::config)
            .configure(routes::user_routes::config)
            .configure(routes::message_routes::config)
            .configure(routes::message_score_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}