use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize)]
pub struct UserProfileModel {
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UserProfileUpdateRequestModel {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct UserPasswordUpdateRequestModel {
    #[serde(rename = "oldPassword")]
    #[validate(length(min = 8, max = 32))]
    pub old_password: String,
    #[serde(rename = "newPassword")]
    #[validate(length(min = 8, max = 32))]
    pub new_password: String,
}