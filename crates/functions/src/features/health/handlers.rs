use actix_web::{HttpResponse, Result};
use serde_json::json;
use http_response::{create_response, HttpCodeW};

/// GET /health
/// Health check endpoint (no authentication required)
pub async fn health_check() -> Result<HttpResponse> {
    let payload = json!({
        "status": "ok",
        "service": "church-management-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    let resp = create_response(payload, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
