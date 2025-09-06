use actix_web::{web, middleware as mw};

use crate::handlers::statistic_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/statistic")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(statistic_handlers::get_monthly_report)
    );
}