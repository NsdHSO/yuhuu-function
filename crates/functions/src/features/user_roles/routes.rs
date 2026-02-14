use actix_web::web;

use super::handlers;

/// Configure user_roles routes
/// These routes are integrated into users and roles scopes
pub fn configure_user_roles(cfg: &mut web::ServiceConfig) {
    (*cfg).service(
        web::resource("/users/{user_id}/roles")
            .route(web::get().to(handlers::get_user_roles))
            .route(web::post().to(handlers::assign_role))
            .route(web::delete().to(handlers::remove_role))
            .route(web::get().to(handlers::get_users_by_role)),
    );
}
