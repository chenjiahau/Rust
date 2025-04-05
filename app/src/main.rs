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
    let name = (utils::constants::NAME).clone();
    let version = (utils::constants::VERSION).clone();
    let address = (utils::constants::ADDRESS).clone();
    let port: u16 = (utils::constants::PORT).clone();

    // Initialize the database connection
    let db = utils::db_connection::establish_connection(utils::constants::DATABASE_URL.to_string()).await;

    // Initialize the app state
    let app_state = utils::app_state::AppState {
        name,
        version,
        db,
    };

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .configure(routes::app_routes::config)
            .configure(routes::unauth_routes::config)
            .configure(routes::dashboard_routes::config)
            .configure(routes::setting_routes::config)
            .configure(routes::place_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}