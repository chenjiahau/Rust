use serde::{Serialize, Deserialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpendingCategoryModel {
    pub id: Option<i64>,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub order: i32,
    pub budget: f64,
    pub is_default: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct SpendingCategoryRequestModel {
    #[validate (length(min = 1, max = 32))]
    pub name: String,
    #[validate (range(min = 0.0))]
    pub budget: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpendingCategoriesResponseModel {
    pub spending_categories: Vec<SpendingCategoryModel>,
}