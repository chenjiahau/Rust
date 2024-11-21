use actix_web::{error, Error};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use serde::Serialize;
use crate::utils::{api_response, jwt::decode_jwt};

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
        return Err(Error::from(api_response::ApiResponse::new(401, serde_json::to_string(&error_response).unwrap())));
    }

    // Check if the Authorization header is in the correct format
    let str_ary = auth.unwrap().to_str().unwrap().split(" ").collect::<Vec<&str>>();
    if str_ary.len() != 2 {
        return Err(Error::from(api_response::ApiResponse::new(401, serde_json::to_string(&error_response).unwrap())));
    }

    if str_ary[0] != "Bearer" {
        return Err(Error::from(api_response::ApiResponse::new(401, serde_json::to_string(&error_response).unwrap())));
    }

    // Check if the token has expired
    let token = str_ary[1];
    let claim = decode_jwt(token.to_string()).unwrap();
    let now = chrono::Utc::now().timestamp() as usize;

    if claim.claims.exp < now {
        return Err(Error::from(api_response::ApiResponse::new(401, serde_json::to_string(&error_response).unwrap())));
    }

    next.call(req).await
        .map_err(| _err| {
            error::ErrorUnauthorized("Unauthorized")
        })
}