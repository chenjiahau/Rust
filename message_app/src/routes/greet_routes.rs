use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/greet")
            .service(handlers::greet_handlers::greet)
            .service(handlers::greet_handlers::greet_message)
    );
}