use actix_web::{middleware as mw, web};
use super::handlers;
use super::middlewares;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/unauth")
                    .service(handlers::uauth_handlers::register)
                    .service(handlers::uauth_handlers::login)
            )
            .service(
                web::scope("/auth")
                    .wrap(mw::from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .service(
                        web::scope("/user")
                            .service(handlers::user_handlers::user)
                            .service(handlers::user_handlers::update_user)
                    )
            )
    );
}