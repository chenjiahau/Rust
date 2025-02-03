mod utils;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("App")
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
    })
    .bind((address, port))?
    .run()
    .await
}