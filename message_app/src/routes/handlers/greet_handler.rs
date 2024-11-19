use actix_web::{get, Responder};
use crate::utils::api_response;

#[get("/greet")]
async fn greet() -> impl Responder {
    api_response::ApiResponse::new(200, "Hello, World!".to_string())
}
