use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};
use crate::utils::{app_state::{self, AppState}, api_response};
use serde::Serialize;

// Return a simple greeting message
#[get("/greet")]
async fn greet(app_state: web::Data::<AppState>) -> impl Responder {
    let res = app_state.db
    .query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT * FROM user")).await.unwrap();

    api_response::ApiResponse::new(200, "Hello, World!".to_string())
}

#[derive(Serialize)]
struct GreetData {
    message: String
}

// Return a simple greeting message in JSON format
#[get("/greet/message")]
async fn greet_message() -> impl Responder {
    let greet_data = GreetData { message: "Hello, World!".to_string() };
    api_response::json_response(&greet_data, 200)
}