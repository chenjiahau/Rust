use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserProfileModel {
    pub name: String,
    pub avatar: Option<String>,
    pub security_password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UserProfileUpdateRequestModel {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    pub security_password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UserPasswordUpdateRequestModel {
    #[serde(rename = "oldPassword")]
    #[validate(length(min = 8, max = 32))]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    #[validate(length(min = 8, max = 32))]
    pub new_password: String,
}