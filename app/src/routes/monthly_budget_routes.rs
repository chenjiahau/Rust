use actix_web::{web, middleware as mw};

use crate::handlers::monthly_budget_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/monthly_budget")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(monthly_budget_handlers::get_monthly_budget_count)
            .service(monthly_budget_handlers::get_monthly_budget)
            .service(monthly_budget_handlers::create_monthly_budget)
  );
}