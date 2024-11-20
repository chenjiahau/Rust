use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::greet_handler::greet)
            .service(handlers::greet_handler::greet_message)
    );
}