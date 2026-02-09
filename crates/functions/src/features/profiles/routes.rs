use actix_web::web;

use super::handlers;

pub fn configure_profiles(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/profile")
            .route(web::post().to(handlers::create_profile))
            .route(web::put().to(handlers::update_profile))
            .route(web::get().to(handlers::get_profile)),
    );
}
