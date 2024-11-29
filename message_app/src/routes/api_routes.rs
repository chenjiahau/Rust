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
                            .service(handlers::user_handlers::get_all_users)
                            .service(handlers::user_handlers::get_user_by_id)
                            .service(handlers::user_handlers::delete_user)
                    )
                    .service(
                        web::scope("/role")
                            .service(handlers::role_handlers::get_all_role)
                            .service(handlers::role_handlers::get_role)
                            .service(handlers::role_handlers::create_role)
                            .service(handlers::role_handlers::update_role)
                            .service(handlers::role_handlers::delete_role)
                    )
            )
    );
}