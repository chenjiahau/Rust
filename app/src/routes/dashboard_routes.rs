use actix_web::{web, middleware as mw};

use crate::handlers::dashboard_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dashboard")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(dashboard_handlers::dashboard)
  );
}