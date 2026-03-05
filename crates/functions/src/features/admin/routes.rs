use actix_web::web;

use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/users/search", web::get().to(handlers::search_users))
            .service(
                web::scope("/users/{user_id}")
                    // Family Relationships
                    .route("/family", web::get().to(handlers::get_user_family))
                    .route("/family", web::post().to(handlers::create_user_family))
                    .route(
                        "/family/{id}",
                        web::get().to(handlers::get_user_family_by_id),
                    )
                    .route("/family/{id}", web::put().to(handlers::update_user_family))
                    .route(
                        "/family/{id}",
                        web::delete().to(handlers::delete_user_family),
                    )
                    // Spiritual Milestones
                    .route("/milestones", web::get().to(handlers::get_user_milestones))
                    .route(
                        "/milestones",
                        web::post().to(handlers::create_user_milestone),
                    )
                    .route(
                        "/milestones/{id}",
                        web::get().to(handlers::get_user_milestone_by_id),
                    )
                    .route(
                        "/milestones/{id}",
                        web::put().to(handlers::update_user_milestone),
                    )
                    .route(
                        "/milestones/{id}",
                        web::delete().to(handlers::delete_user_milestone),
                    )
                    // Membership History
                    .route(
                        "/membership-history",
                        web::get().to(handlers::get_user_membership_history),
                    )
                    .route(
                        "/membership-history",
                        web::post().to(handlers::create_user_membership_history),
                    )
                    .route(
                        "/membership-history/{id}",
                        web::get().to(handlers::get_user_membership_history_by_id),
                    )
                    .route(
                        "/membership-history/{id}",
                        web::put().to(handlers::update_user_membership_history),
                    )
                    .route(
                        "/membership-history/{id}",
                        web::delete().to(handlers::delete_user_membership_history),
                    )
                    // User Skills
                    .route("/skills", web::get().to(handlers::get_user_skills))
                    .route("/skills", web::post().to(handlers::create_user_skill))
                    .route(
                        "/skills/{id}",
                        web::get().to(handlers::get_user_skill_by_id),
                    )
                    .route("/skills/{id}", web::put().to(handlers::update_user_skill))
                    .route(
                        "/skills/{id}",
                        web::delete().to(handlers::delete_user_skill),
                    ),
            ),
    );
}
