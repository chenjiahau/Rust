use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};
use crate::utils::{app_state::{self, AppState}, api_response};

#[get("/greet")]
async fn greet(app_state: web::Data::<AppState>) -> impl Responder {
    let res = app_state.db
    .query_all(Statement::from_string(sea_orm::DatabaseBackend::Postgres, "SELECT * FROM user")).await.unwrap();

    api_response::ApiResponse::new(200, "Hello, World!".to_string())
}
