use actix_web::{middleware as mw, web};

use crate::handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/message_scores")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(handlers::message_score_handlers::create_message_score)
            .service(handlers::message_score_handlers::get_all_message_scores)
            .service(handlers::message_score_handlers::get_message_score)
            .service(handlers::message_score_handlers::update_message_score)
            .service(handlers::message_score_handlers::delete_message_score)
    );
}