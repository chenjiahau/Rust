use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/unauth")
            .service(handlers::unauth_handlers::signup)
            .service(handlers::unauth_handlers::signin)
            .service(handlers::unauth_handlers::create_new_password_by_email)
    );
}