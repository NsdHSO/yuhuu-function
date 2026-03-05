use actix_web::web;

use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profiles/me/family")
            .route("", web::post().to(handlers::create_family_relationship))
            .route("", web::get().to(handlers::list_family_relationships))
            .route("/{id}", web::get().to(handlers::get_family_relationship))
            .route("/{id}", web::put().to(handlers::update_family_relationship))
            .route(
                "/{id}",
                web::delete().to(handlers::delete_family_relationship),
            ),
    );
}
