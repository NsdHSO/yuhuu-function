use actix_web::web;

use super::handlers;

/// Configure user_roles routes
/// These routes are integrated into users and roles scopes
pub fn configure_user_roles(cfg: &mut web::ServiceConfig) {
    // Routes for managing user roles
    cfg.route(
        "/users/{user_id}/roles",
        web::get().to(handlers::get_user_roles),
    )
    .route(
        "/users/{user_id}/roles",
        web::post().to(handlers::assign_role),
    )
    .route(
        "/users/{user_id}/roles/{role_id}",
        web::delete().to(handlers::remove_role),
    )
    .route(
        "/roles/{role_id}/users",
        web::get().to(handlers::get_users_by_role),
    );
}
