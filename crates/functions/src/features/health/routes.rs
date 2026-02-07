use actix_web::web;

use super::handlers;

/// Configure health routes
pub fn configure_health(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handlers::health_check));
}
