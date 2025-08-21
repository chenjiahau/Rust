use actix_web::{web, middleware as mw};

use crate::handlers::spending_category_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/spending_category")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(spending_category_handlers::get_spending_categories)
            .service(spending_category_handlers::get_spending_category)
            .service(spending_category_handlers::create_spending_category)
            .service(spending_category_handlers::update_spending_category)
            .service(spending_category_handlers::delete_spending_category)
  );
}