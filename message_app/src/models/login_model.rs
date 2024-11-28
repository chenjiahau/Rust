use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct LoginRequestModel {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String
}

#[derive(Serialize)]
pub struct LoginModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub token: String
}

