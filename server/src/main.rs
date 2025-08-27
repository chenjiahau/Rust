mod utils;
mod handlers;
mod routes;
mod models;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use actix_files::Files;
use actix_web::{web, middleware::Logger, App, HttpServer};
use utils::swagger::ApiDoc;

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
    let static_path = (utils::constants::STATIC_PATH).clone();

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
            .service(Files::new("/static", static_path.clone()).show_files_listing())
            .configure(routes::app_routes::config)
            .configure(routes::unauth_routes::config)
            .configure(routes::user_routes::config)
            .configure(routes::dashboard_routes::config)
            .configure(routes::setting_routes::config)
            .configure(routes::place_routes::config)
            .configure(routes::spending_category_routes::config)
            .configure(routes::consumption_routes::config)
            .configure(routes::monthly_budget_routes::config)
            .configure(routes::monthly_setting_routes::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind((address, port))?
    .run()
    .await
}