use actix_web::web;

use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/profiles/me/milestones")
            .route("", web::post().to(handlers::create_milestone))
            .route("", web::get().to(handlers::list_milestones))
            .route("/{id}", web::get().to(handlers::get_milestone))
            .route("/{id}", web::put().to(handlers::update_milestone))
            .route("/{id}", web::delete().to(handlers::delete_milestone)),
    );
}
