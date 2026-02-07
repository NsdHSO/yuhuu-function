use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use http_response::{create_response, HttpCodeW};
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::client::AuthClient;
use crate::user_context::UserContext;

/// Middleware for JWT authentication
pub struct AuthMiddleware {
    auth_client: Rc<AuthClient>,
    public_paths: Vec<String>,
}

impl AuthMiddleware {
    /// Create new auth middleware
    pub fn new(auth_client: AuthClient) -> Self {
        Self {
            auth_client: Rc::new(auth_client),
            public_paths: vec![
                "/health".to_string(),
                "/api/health".to_string(),
                "/playground".to_string(),
            ],
        }
    }

    /// Add public paths that don't require authentication
    pub fn with_public_paths(mut self, paths: Vec<String>) -> Self {
        self.public_paths.extend(paths);
        self
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            auth_client: self.auth_client.clone(),
            public_paths: self.public_paths.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    auth_client: Rc<AuthClient>,
    public_paths: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();

        // Check if path is public
        let is_public = self
            .public_paths
            .iter()
            .any(|p| path.starts_with(p));

        if is_public {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_boxed_body())
            });
        }

        // Extract Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let service = self.service.clone();
        let auth_client = self.auth_client.clone();

        Box::pin(async move {
            // Check for Authorization header
            let token = match auth_header {
                Some(header) if header.starts_with("Bearer ") => {
                    header.trim_start_matches("Bearer ").to_string()
                }
                _ => {
                    let response = create_response(
                        "Missing or invalid Authorization header",
                        HttpCodeW::Unauthorized
                    );
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized().json(response)
                    ));
                }
            };

            // Verify token with auth server
            let verification = match auth_client.verify_token(&token).await {
                Ok(v) => v,
                Err(e) => {
                    let response = create_response(
                        format!("Token verification failed: {}", e),
                        HttpCodeW::Unauthorized
                    );
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized().json(response)
                    ));
                }
            };

            // Check if token is valid
            if !verification.valid {
                let response = create_response("Invalid token", HttpCodeW::Unauthorized);
                return Ok(req.into_response(
                    HttpResponse::Unauthorized().json(response)
                ));
            }

            // Extract user info from verification
            let user_context = UserContext {
                auth_user_id: verification.user_id.unwrap_or_default(),
                email: verification.email,
                role: verification.role.unwrap_or_else(|| "user".to_string()),
                token: token.clone(),
            };

            // Insert user context into request extensions
            req.extensions_mut().insert(user_context);

            // Call the next service
            let res = service.call(req).await?;
            Ok(res.map_into_boxed_body())
        })
    }
}
