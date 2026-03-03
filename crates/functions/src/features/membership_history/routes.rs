use actix_web::web;

use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/profiles/me/membership-history")
            .route("", web::post().to(handlers::create_membership))
            .route("", web::get().to(handlers::list_memberships))
            .route("/{id}", web::get().to(handlers::get_membership))
            .route("/{id}", web::put().to(handlers::update_membership))
            .route("/{id}", web::delete().to(handlers::delete_membership)),
    );
}
