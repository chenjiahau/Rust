use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;

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

#[derive(Debug, Deserialize, Validate)]
pub struct SigninRequestModel {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenModel {
    pub token: String,
    #[serde(rename = "expirationTime")]
    pub expiration_time: usize,
    #[serde(rename = "startTime")]
    pub start_time: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct SigninResponseModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub token: Option<TokenModel>,
}