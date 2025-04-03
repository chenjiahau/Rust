use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct SettingResponseModel {
    pub default_income: i64,
    pub target_deposit: i64,
    pub updated_at: String,
}

#[derive(Deserialize, Validate)]
pub struct SettingRequestModel {
    pub default_income: i64,
    pub target_deposit: i64,
}