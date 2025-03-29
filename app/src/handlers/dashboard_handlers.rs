use actix_web::{get, Responder};
use httpstatus::StatusCode;

use crate::utils::api_response::response;

#[get("")]
async fn dashboard() -> impl Responder {
    response::<Option<String>>(StatusCode::Ok, None, None)
}