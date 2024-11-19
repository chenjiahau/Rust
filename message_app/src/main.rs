mod utils;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Message app")
}

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
            .service(
                web::scope("/api") // Set the scope of the routes
                    .service(greet)
            )
    })
    .bind((address, port))? // bind the server to the address and port
    .run()
    .await
}