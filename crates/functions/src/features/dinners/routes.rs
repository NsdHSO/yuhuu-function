use actix_web::web;

use super::handlers;

pub fn configure_dinners(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dinners")
            .route("", web::post().to(handlers::create_dinner))
            .route("", web::get().to(handlers::list_dinners))
            .route("/{id}", web::get().to(handlers::get_dinner))
            .route(
                "/{id}/participants",
                web::get().to(handlers::get_dinner_participants),
            )
            .route(
                "/{id}/participants",
                web::post().to(handlers::add_participant),
            )
            .route(
                "/{dinner_id}/participants/{participant_id}",
                web::delete().to(handlers::remove_participant),
            ),
    );
}
