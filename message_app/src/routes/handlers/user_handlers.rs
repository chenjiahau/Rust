use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn user() -> impl Responder {
    let users = vec!["Alice", "Bob", "Charlie"];
    HttpResponse::Ok().json(users)
}