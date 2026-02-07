use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use futures_util::future::{ready, Ready};
use serde::{Deserialize, Serialize};

/// User context attached to authenticated requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    /// User ID from auth server
    pub auth_user_id: String,

    /// User email
    pub email: Option<String>,

    /// User role (admin, user, etc.)
    pub role: String,

    /// JWT token
    pub token: String,
}

impl UserContext {
    /// Check if user has admin role
    pub fn is_admin(&self) -> bool {
        self.role.to_lowercase() == "admin"
    }

    /// Check if user has specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.role.to_lowercase() == role.to_lowercase()
    }
}

/// Extractor for UserContext from request
impl FromRequest for UserContext {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();

        match extensions.get::<UserContext>() {
            Some(user_context) => ready(Ok(user_context.clone())),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "User not authenticated",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_admin() {
        let user = UserContext {
            auth_user_id: "123".to_string(),
            email: Some("test@example.com".to_string()),
            role: "Admin".to_string(),
            token: "token".to_string(),
        };

        assert!(user.is_admin());
    }

    #[test]
    fn test_has_role() {
        let user = UserContext {
            auth_user_id: "123".to_string(),
            email: Some("test@example.com".to_string()),
            role: "Editor".to_string(),
            token: "token".to_string(),
        };

        assert!(user.has_role("editor"));
        assert!(user.has_role("Editor"));
        assert!(!user.has_role("admin"));
    }
}
