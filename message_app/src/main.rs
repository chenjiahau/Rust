use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware;
use serde::{Serialize, Deserialize};

mod middlewares;
mod utils;

#[derive(Debug)]
struct MainError {
    message: String
}

#[derive(Deserialize)]
struct Param {
    name: String
}

#[derive(Serialize)]
struct Data {
    message: String,
}

#[get("/greet/{name}")]
async fn greet(
    param: web::Path<Param>
) -> impl Responder {
    let names = vec!["Alice", "Bob", "Charlie"];

    match names.contains(&param.name.as_str()) {
        true => {
            let data = Data {
                message: format!("Hello, {}!", &param.name),
            };
            utils::api_response::ApiResponse::ok(data).to_http_response()
        }
        false => {
            let data = Data {
                message: "Name not found".to_string(),
            };
            utils::api_response::ApiResponse::not_found(data).to_http_response()
        }
    }
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

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // Log the requests
            .service(
                web::scope("/api") // Set the scope of the routes
                    .wrap(middleware::from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .service(greet)
            )
    })
    .bind((address, port))
    .map_err(|err| MainError { message: err.to_string() })?
    .run()
    .await
    .map_err(|err| MainError { message: err.to_string() })
}