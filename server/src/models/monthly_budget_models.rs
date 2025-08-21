use serde::{Serialize, Deserialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MonthlyBudgetModel {
    pub id: i64,
    pub user_id: Uuid,
    pub month: i32,
    pub year: i32,
    #[serde(rename = "scId")]
    pub spending_category_id: i64,
    pub budget: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MonthlyBudgetCountResponseModel {
    pub count: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MonthlyBudgetResponseModel {
    pub id: i64,
    pub user_id: Uuid,
    pub month: i32,
    pub year: i32,
    pub spending_category_id: i64,
    pub budget: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MonthlyBudgetsResponseModel {
    pub monthly_budgets: Vec<MonthlyBudgetModel>,
}