use actix_web::{web, middleware as mw};

use crate::handlers::monthly_setting_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/monthly_setting")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(monthly_setting_handlers::get_monthly_setting_count)
            .service(monthly_setting_handlers::get_monthly_setting)
            .service(monthly_setting_handlers::create_monthly_setting)
  );
}