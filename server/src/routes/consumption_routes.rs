use actix_web::{web, middleware as mw};

use crate::handlers::consumption_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/consumption")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(consumption_handlers::get_consumptions)
            .service(consumption_handlers::get_consumptions_by_year_and_month)
            .service(consumption_handlers::get_consumption)
            .service(consumption_handlers::create_consumption)
            .service(consumption_handlers::update_consumption)
            .service(consumption_handlers::delete_consumption)
  );
}