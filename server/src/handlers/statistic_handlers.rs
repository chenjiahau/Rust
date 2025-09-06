use actix_web::{web, get, HttpRequest, Responder};
use httpstatus::StatusCode;
use sea_orm::{QueryOrder, RelationTrait, QuerySelect, ColumnTrait, Condition, EntityTrait, QueryFilter, sea_query::Expr as SeaExpr};
use sea_orm::sea_query::Func;
use uuid::Uuid;
use serde::Deserialize;

use entity::{spending_categories, monthly_settings, monthly_budgets, consumptions};
use crate::models::statistic_models;
use crate::utils::app_state::AppState;
use crate::utils::api_response::{get_user_id_from_request, response};
use crate::utils::message;
use crate::utils::date;

#[derive(Debug, Deserialize)]
struct Param {
    year: Option<u32>,
    month: Option<u32>,
}

#[utoipa::path(
    get,
    path = "/api/statistic/monthly_report",
    params(
        ("year" = Option<u32>, Query, description = "Year of the report"),
        ("month" = Option<u32>, Query, description = "Month of the report"),
    ),
    security(("bearerAuth" = [])),
    tag = "Statistic",
)]
#[get("/monthly_report")]
async fn get_monthly_report(
    req: HttpRequest,
    app_state: web::Data::<AppState>,
    query: web::Query<Param>,
) -> impl Responder {
    let user_id = get_user_id_from_request(&req).unwrap();
    let (year, month) = date::get_current_year_and_month();
    let p_year = query.year;
    let p_month = query.month;
    let month_report = statistic_models::MonthlyReportResponseModel {
        user_id: Some(Uuid::parse_str(user_id.as_str()).unwrap()),
        year: p_year.unwrap_or(year),
        month: p_month.unwrap_or(month),
        income: 0,
        deposit: 0,
        remaining_deposit: 0,
        total_expense: 0,
        expense_to_income_ratio: 0.0,
        budgets: vec![],
    };

    // Find the monthly setting
    let result = monthly_settings::Entity::find()
        .filter(
            Condition::all()
                .add(monthly_settings::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
                .add(monthly_settings::Column::Year.eq(month_report.year))
                .add(monthly_settings::Column::Month.eq(month_report.month))
        )
        .one(&app_state.db)
        .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let option_model = result.unwrap();
    if option_model.is_none() {
        let error_message = message::ErrorMessage::MonthlySettingNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let model = option_model.unwrap();
    let mut month_report = month_report;
    month_report.income = model.income;
    month_report.deposit = model.deposit as i64;
    month_report.remaining_deposit = model.income - model.deposit as i64;

    // Find the monthly budget
    let result = monthly_budgets::Entity::find()
    .select_only()
    .column(spending_categories::Column::Id)
    .column(spending_categories::Column::Name)
    .column(monthly_budgets::Column::Budget)
    .join(
        sea_orm::JoinType::InnerJoin,
        monthly_budgets::Relation::SpendingCategories.def(),
    )
    .filter(monthly_budgets::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
    .filter(monthly_budgets::Column::Year.eq(month_report.year))
    .filter(monthly_budgets::Column::Month.eq(month_report.month))
    .order_by_asc(monthly_budgets::Column::Id)
    .into_tuple::<(i64, String, f64)>()
    .all(&app_state.db)
    .await;

    if result.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let models = result.unwrap();
    if models.is_empty() {
        let error_message = message::ErrorMessage::MonthlyBudgetNotFound;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let spending_category_models = models.into_iter().map(|m| {
        statistic_models::BudgetStatisticModel {
            spending_category_id: m.0,
            spending_category_name: m.1,
            budget: m.2 as i64,
            expense: 0,
            percentage_used: 0.0,
        }
    }).collect::<Vec<_>>();
    
    // Find total spending per category for the month
    let results = consumptions::Entity::find()
        .select_only()
        .column(consumptions::Column::SpendingCategoryId)
        .expr_as(
            Func::sum(SeaExpr::col(consumptions::Column::Amount)),
            "total_amount"
        )
        .filter(consumptions::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).unwrap()))
        .filter(consumptions::Column::Date.like(format!("{}{:02}%", month_report.year, month_report.month)))
        .group_by(consumptions::Column::SpendingCategoryId)
        .into_tuple::<(i64, f64)>()
        .all(&app_state.db)
        .await;

    if results.is_err() {
        let error_message = message::ErrorMessage::InternalServerError;
        return response(
            StatusCode::NotFound,
            error_message.to_code(),
            Some(error_message.to_string()),
            Option::<()>::None,
        );
    }

    let mut total_expense: i64 = 0;
    let spending_map: std::collections::HashMap<i64, f64> = results.unwrap().into_iter().collect();
    let mut budgets = vec![];
    for mut budget in spending_category_models {
        if let Some(&expense) = spending_map.get(&budget.spending_category_id) {
            budget.expense = expense as i64;
            budget.percentage_used = if budget.budget == 0 {
                0.0
            } else {
               ((expense / budget.budget as f64) * 100.0) / 100.0
            };

            total_expense += expense as i64;
        }
        budgets.push(budget);
    }
    month_report.budgets = budgets;
    month_report.total_expense = total_expense;
    month_report.expense_to_income_ratio = total_expense as f64 / month_report.income as f64;

    // Return monthly report
    let success_message = message::SuccessMessage::Success;
    response(
        StatusCode::Ok,
        success_message.to_code(),
        Some(success_message.to_string()),
        Some(month_report),
    )
}
