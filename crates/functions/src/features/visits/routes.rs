use actix_web::web;

use super::handlers::user_assignments;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/visits")
            .route("/my-assignments", web::get().to(user_assignments::list_my_assignments))
            .route("/assignments/{id}", web::get().to(user_assignments::get_assignment))
            .route("/assignments/{id}", web::put().to(user_assignments::update_assignment))
            .route("/assignments/{id}/arrive", web::post().to(user_assignments::mark_arrival))
            .route("/assignments/{id}/complete", web::post().to(user_assignments::mark_complete)),
    );
}
