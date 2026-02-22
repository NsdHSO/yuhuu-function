use actix_web::web;

use super::handlers;

pub fn configure_profiles(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/me/profile")
                .route(web::get().to(handlers::get_my_profile))
                .route(web::post().to(handlers::create_my_profile))
                .route(web::put().to(handlers::update_my_profile))
        )
        .service(
            web::resource("/users/{id}/profile")
                .route(web::post().to(handlers::create_profile))
                .route(web::put().to(handlers::update_profile))
                .route(web::get().to(handlers::get_profile)),
        );
}
