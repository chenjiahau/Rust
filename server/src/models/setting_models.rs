use serde::{Serialize, Deserialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SettingResponseModel {
    pub default_income: i64,
    pub target_deposit: i64,
    pub updated_at: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct SettingRequestModel {
    pub default_income: i64,
    pub target_deposit: i64,
}