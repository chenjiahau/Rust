use actix_web::{error, web, Error, HttpMessage};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use httpstatus::StatusCode;
use sea_orm::{EntityTrait, QueryFilter, Condition, ColumnTrait};

use crate::utils::jwt::decode_jwt;
use crate::utils::api_response::response;


fn return_unauthorized() -> Error {
    let error_message = crate::utils::message::ErrorMessage::Unauthorized;
    let response = response::<Option<String>>(
        StatusCode::Unauthorized,
        error_message.to_code(),
        Some(error_message.to_string()),
        None,
    );

    error::InternalError::from_response(
        error::ErrorUnauthorized("Unauthorized"),
        response,
    ).into()
}

pub async fn check_auth_middleware (
    app_state: web::Data<crate::utils::app_state::AppState>,
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

    let token = str_ary[1];

    // Check if the token is valid
    let _claim = match decode_jwt(token.to_string()) {
        Ok(claim) => claim,
        Err(_) => {
            return Err(return_unauthorized());
        }
    };

    // Check if the token is not expired
    let result = entity::tokens::Entity::find()
        .filter(
            Condition::all()
                .add(entity::tokens::Column::Token.eq(token))
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if result.is_none() {
        return Err(return_unauthorized());
    }

    let expiration_time = result.unwrap().expiration_time as usize;
    let now = chrono::Utc::now().timestamp() as usize;
    if expiration_time < now {
        return Err(return_unauthorized());
    }

    // Pass the user id to handlers
    req.extensions_mut().insert::<String>(_claim.claims.id.to_string());
    next.call(req).await
}