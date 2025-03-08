use actix_web::{get, Responder};
use httpstatus::StatusCode;

use crate::models::user_models;
use crate::utils::api_response::response_with_data;

#[get("")]
async fn roles() -> impl Responder {
    let role_list = user_models::RoleResponseModel {
        roles: vec![
            user_models::RoleModel {
                name: "Admin".to_string(),
                value: 1,
            },
            user_models::RoleModel {
                name: "User".to_string(),
                value: 2,
            },
        ],
    };

    response_with_data(StatusCode::Ok, "Success", Some(role_list))
}
