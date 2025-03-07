use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequestModel {
    #[validate (length(min = 8, max = 32))]
    pub name: Option<String>,
    #[validate (email)]
    pub email: Option<String>,
    #[validate (length(min = 8, max = 32))]
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignupResponseModel {
    pub name: String,
    pub email: String,
}
