use actix_web::web;

use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/profiles/me/skills")
            .route("", web::post().to(handlers::create_skill))
            .route("", web::get().to(handlers::list_skills))
            .route("/{id}", web::get().to(handlers::get_skill))
            .route("/{id}", web::put().to(handlers::update_skill))
            .route("/{id}", web::delete().to(handlers::delete_skill)),
    );
}
