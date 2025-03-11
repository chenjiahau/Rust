use actix_web::{middleware as mw, web};

use crate::handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/messages")
            .wrap(mw::from_fn(middlewares::auth_middlewares::check_auth_middleware))
            .service(handlers::message_handlers::create_message)
            .service(handlers::message_handlers::get_messages_by_user)
            .service(handlers::message_handlers::update_message)
            .service(handlers::message_handlers::delete_messages_by_user)
            .service(handlers::message_handlers::delete_message_by_user)
    );
}