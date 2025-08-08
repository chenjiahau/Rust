use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TokenModel {
    pub token: String,
    #[serde(rename = "expirationTime")]
    pub expiration_time: usize,
    #[serde(rename = "startTime")]
    pub start_time: usize,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SignupRequestModel {
    #[validate (length(min = 8, max = 32))]
    pub name: Option<String>,
    #[validate (email)]
    pub email: Option<String>,
    #[validate (length(min = 8, max = 32))]
    pub password: Option<String>,
}

#[derive(Debug, Serialize,ToSchema)]
pub struct SignupResponseModel {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct SigninRequestModel {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}

#[derive(Serialize, ToSchema )]
pub struct SigninResponseModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub token: Option<TokenModel>,
    pub registered_at: String,
}
