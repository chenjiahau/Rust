use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/unauth")
            .service(handlers::unauth_handlers::signup)
            .service(handlers::unauth_handlers::signin)
    );
}