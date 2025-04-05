use actix_web::{web, middleware as mw};

use crate::handlers::place_handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/place")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(place_handlers::get_places)
            .service(place_handlers::create_place)
            .service(place_handlers::update_place)
            .service(place_handlers::delete_place)
  );
}