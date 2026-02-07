use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Client for communicating with the auth server
#[derive(Clone)]
pub struct AuthClient {
    base_url: String,
    api_key: Option<String>,
    client: reqwest::Client,
}

/// User data from auth server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub status: String,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Token verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenVerification {
    pub valid: bool,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub exp: Option<i64>,
}

impl AuthClient {
    /// Create a new auth client
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    /// Get user by ID from auth server
    pub async fn get_user(&self, user_id: &str) -> Result<AuthUser> {
        let url = format!("{}/api/users/{}", self.base_url, user_id);

        let mut request = self.client.get(&url);
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to auth server")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Auth server returned error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            );
        }

        response
            .json::<AuthUser>()
            .await
            .context("Failed to parse auth user response")
    }

    /// Get user by email from auth server
    pub async fn get_user_by_email(&self, email: &str) -> Result<AuthUser> {
        let url = format!("{}/api/users/email/{}", self.base_url, email);

        let mut request = self.client.get(&url);
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to auth server")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Auth server returned error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            );
        }

        response
            .json::<AuthUser>()
            .await
            .context("Failed to parse auth user response")
    }

    /// Verify JWT token with auth server
    pub async fn verify_token(&self, token: &str) -> Result<TokenVerification> {
        let url = format!("{}/api/auth/verify", self.base_url);

        let mut request = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token));

        if let Some(api_key) = &self.api_key {
            request = request.header("X-API-Key", api_key);
        }

        let response = request
            .send()
            .await
            .context("Failed to send token verification request")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Token verification failed: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            );
        }

        response
            .json::<TokenVerification>()
            .await
            .context("Failed to parse token verification response")
    }

    /// Check if user exists in auth server
    pub async fn user_exists(&self, user_id: &str) -> Result<bool> {
        match self.get_user(user_id).await {
            Ok(_) => Ok(true),
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("404") || err_msg.contains("not found") {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_client_creation() {
        let client = AuthClient::new("http://localhost:8080".to_string(), None);
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_auth_client_url_trimming() {
        let client = AuthClient::new("http://localhost:8080/".to_string(), None);
        assert_eq!(client.base_url, "http://localhost:8080");
    }
}
