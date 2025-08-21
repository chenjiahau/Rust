use actix_web::{get, Responder};
use httpstatus::StatusCode;

use crate::utils::api_response::response;
use crate::utils::message;

#[get("")]
async fn dashboard() -> impl Responder {
    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Option::<()>::None,
    )
}