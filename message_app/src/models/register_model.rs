use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequestModel {
    #[validate (length(min = 8, max = 32))]
    pub name: String,
    #[validate (email)]
    pub email: String,
    #[validate (length(min = 8, max = 32))]
    pub password: String
}

#[derive(Serialize)]
pub struct RegisterModel {
    pub name: String,
    pub email: String,
}