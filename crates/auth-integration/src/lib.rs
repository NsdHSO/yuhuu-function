//! # Auth Integration Crate
//!
//! This crate provides authentication integration with the external auth server.
//! It handles JWT verification, user context management, and provides middleware
//! for protecting routes.
//!
//! ## Features
//!
//! - **AuthClient**: HTTP client for communicating with the auth server
//! - **AuthMiddleware**: Actix-web middleware for JWT authentication
//! - **UserContext**: Request-scoped user information
//! - **UserService**: Service layer combining auth and local data
//!
//! ## Usage
//!
//! ```rust,ignore
//! use auth_integration::{AuthClient, AuthMiddleware, UserService};
//! use actix_web::{web, App, HttpServer};
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     // Create auth client
//!     let auth_client = AuthClient::new(
//!         "http://localhost:8080".to_string(),
//!         Some("api_key".to_string())
//!     );
//!
//!     // Create user service
//!     let user_service = UserService::new(db.clone(), auth_client.clone());
//!
//!     HttpServer::new(move || {
//!         App::new()
//!             .app_data(web::Data::new(user_service.clone()))
//!             .wrap(AuthMiddleware::new(auth_client.clone()))
//!             .service(your_routes)
//!     })
//!     .bind(("127.0.0.1", 8080))?
//!     .run()
//!     .await
//! }
//! ```

pub mod client;
pub mod middleware;
pub mod service;
pub mod user_context;

pub use client::{AuthClient, AuthUser, TokenVerification};
pub use middleware::AuthMiddleware;
pub use service::{CompleteUser, UserService};
pub use user_context::UserContext;
