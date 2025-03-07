use std::collections::HashMap;

use actix_web::{get, Responder};
use httpstatus::StatusCode;

use crate::utils::api_response::{
    response,
    response_with_data,
    response_error,
    response_not_found,
    response_bad_request
};

#[get("/greet")]
async fn greet() -> impl Responder {
    response(StatusCode::Ok, "Hello, World!")
}

#[get("/version")]
async fn version() -> impl Responder {
    let mut data = HashMap::new();
    data.insert("app", "Rust Actix Web");
    data.insert("version", "1.0.0");

    // response_with_data(StatusCode::Ok, "Success", Some({}))
    response_with_data(StatusCode::Ok, "Success", Some(data))
}

#[get("/error")]
async fn error() -> impl Responder {
    response_error(StatusCode::InternalServerError, "Internal Server Error")
}

#[get("/notfound")]
async fn not_found() -> impl Responder {
    response_not_found(StatusCode::NotFound, "Not Found")
}

#[get("/badrequest")]
async fn bad_request() -> impl Responder {
    response_bad_request(StatusCode::BadRequest, "Bad Request")
}