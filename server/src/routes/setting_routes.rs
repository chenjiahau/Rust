use actix_web::{web, middleware as mw};

use crate::handlers::setting_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/setting")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(setting_handlers::get_setting)
            .service(setting_handlers::update_setting)
  );
}