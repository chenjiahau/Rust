use actix_web::{web, middleware as mw};

use crate::handlers::user_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/user")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(user_handlers::get_user_profile)
            .service(user_handlers::update_user_profile)
            .service(user_handlers::update_user_password)
            .service(user_handlers::upload_user_avatar)
  );
}