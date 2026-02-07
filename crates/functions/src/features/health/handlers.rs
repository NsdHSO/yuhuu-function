use actix_web::{HttpResponse, Result};
use serde_json::json;

/// GET /health
/// Health check endpoint (no authentication required)
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "service": "church-management-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
