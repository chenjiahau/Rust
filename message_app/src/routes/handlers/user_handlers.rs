use actix_web::{get, HttpResponse, Responder};
use crate::utils::api_response;

#[get("")]
pub async fn user() -> impl Responder {
    let users = vec!["Alice", "Bob", "Charlie"];
    api_response::ApiResponse::new(200, serde_json::to_string(&users).unwrap())
}