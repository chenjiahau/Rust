use actix_web::{middleware as mw, web};

use crate::handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(
                web::scope("/roles")
                    .service(handlers::user_handlers::roles)
            )
    );
}