use serde::Serialize;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BudgetStatisticModel {
    pub spending_category_id: i64,
    pub spending_category_name: String,
    pub budget: i64,
    pub expense: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MonthlyReportResponseModel {
    pub user_id: Option<Uuid>,
    pub month: u32,
    pub year: u32,
    pub income: i64,
    pub deposit: i64,
    pub remaining_deposit: i64,
    pub total_expense: i64,
    pub expense_to_income_ratio: f64,
    pub budgets: Vec<BudgetStatisticModel>,
}