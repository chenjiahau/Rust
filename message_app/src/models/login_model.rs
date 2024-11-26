use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequestModel {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String
}

#[derive(Serialize)]
pub struct LoginModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub token: String
}

