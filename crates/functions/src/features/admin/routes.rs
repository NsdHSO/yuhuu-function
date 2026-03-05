use actix_web::web;

use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/users/search", web::get().to(handlers::search_users))
            .service(
                web::scope("/users/{user_id}")
                    .route("/family", web::get().to(handlers::get_user_family))
                    .route("/milestones", web::get().to(handlers::get_user_milestones))
                    .route(
                        "/membership-history",
                        web::get().to(handlers::get_user_membership_history),
                    )
                    .route("/skills", web::get().to(handlers::get_user_skills)),
            ),
    );
}
