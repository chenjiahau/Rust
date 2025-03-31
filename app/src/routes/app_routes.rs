use actix_web::web;
use crate::handlers::app_handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/app")
            .service(app_handlers::app)
            .service(app_handlers::index)
            .service(app_handlers::bad_request)
            .service(app_handlers::not_found)
            .service(app_handlers::internal_server_error)
  );
}