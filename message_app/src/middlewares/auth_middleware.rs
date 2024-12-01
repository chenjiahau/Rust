use actix_web::error::InternalError;
use actix_web::{HttpMessage, error, Error};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use serde::Serialize;
use crate::utils;

#[derive(Serialize)]
struct DataResponse {
    message: String
}

pub async fn check_auth_middleware (
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get("Authorization");
    let error_response = DataResponse {
        message: "Unauthorized".to_string()
    };

    // Check if the Authorization header is present
    if auth.is_none() {
        return Err(
          InternalError::from_response(
              error::ErrorUnauthorized("Unauthorized"),
              utils::api_response::ApiResponse::unauthorized(error_response).to_http_response()
          ).into()
        );
    }

    next.call(req).await
        .map_err(| _err| {
            error::ErrorUnauthorized("Unauthorized")
        })
}