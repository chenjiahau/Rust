use actix_web::web;
use crate::handlers::basic_handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/basic")
      .service(basic_handlers::greet)
      .service(basic_handlers::version)
      .service(basic_handlers::error)
      .service(basic_handlers::not_found)
      .service(basic_handlers::bad_request)
  );
}