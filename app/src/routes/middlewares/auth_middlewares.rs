use actix_web::{HttpMessage, error, Error};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use httpstatus::StatusCode;

use crate::utils::jwt_token::decode_jwt;
use crate::utils::api_response::response_bad_request;

fn return_unauthorized() -> Error {
    error::InternalError::from_response(
        error::ErrorUnauthorized("Unauthorized"),
        response_bad_request(StatusCode::Unauthorized, "Unauthorized")
    ).into()
}

pub async fn check_auth_middleware (
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_str = req.headers().get("Authorization");

    // Check if the Authorization header is present
    if auth_str.is_none() {
        return Err(return_unauthorized());
    }

    // Check if the Authorization header is in the correct format
    let str_ary = auth_str.unwrap().to_str().unwrap().split(" ").collect::<Vec<&str>>();
    if str_ary.len() != 2 {
        return Err(return_unauthorized());
    }

    if str_ary[0] != "Bearer" {
        return Err(return_unauthorized());
    }

    // Check if the claims are valid and not expired
    let token = str_ary[1];
    let claim = match decode_jwt(token.to_string()) {
        Ok(claim) => claim,
        Err(_) => {
            return Err(return_unauthorized());
        }
    };
    let now = chrono::Utc::now().timestamp() as usize;

    if claim.claims.exp < now {
        return Err(return_unauthorized());
    }

    req.extensions_mut().insert(claim.claims);

    next.call(req).await
        .map_err(| _err| {
            error::ErrorUnauthorized("Unauthorized")
        })
}