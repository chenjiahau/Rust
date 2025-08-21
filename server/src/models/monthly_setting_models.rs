use serde::{Serialize, Deserialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MonthlySettingModel {
    pub id: i64,
    pub user_id: Uuid,
    pub month: i32,
    pub year: i32,
    pub income: i64,
    pub deposit: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MonthlySettingCountResponseModel {
    pub count: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MonthlySettingResponseModel {
    pub id: i64,
    pub user_id: Uuid,
    pub month: i32,
    pub year: i32,
    pub income: i64,
    pub deposit: f64,
    pub created_at: String,
    pub updated_at: String,
}
