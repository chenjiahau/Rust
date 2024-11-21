use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/uauth")
                    .service(handlers::uauth_handlers::register)
                    .service(handlers::uauth_handlers::login)
            )
    );
}