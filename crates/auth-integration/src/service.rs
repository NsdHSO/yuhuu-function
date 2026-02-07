use anyhow::{Context, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::client::{AuthClient, AuthUser};
use models::dto::{user, User, UserModel};

/// Service for managing users with auth server integration
#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
    auth_client: AuthClient,
}

impl UserService {
    /// Create new user service
    pub fn new(db: DatabaseConnection, auth_client: AuthClient) -> Self {
        Self { db, auth_client }
    }

    /// Get complete user data (auth + local)
    pub async fn get_complete_user(&self, church_user_id: i64) -> Result<CompleteUser> {
        // Get church user record
        let church_user = User::find_by_id(church_user_id)
            .one(&self.db)
            .await
            .context("Failed to query church user")?
            .context("Church user not found")?;

        // Get auth user from auth server
        let auth_user = self
            .auth_client
            .get_user(&church_user.auth_user_id)
            .await
            .context("Failed to fetch auth user")?;

        Ok(CompleteUser {
            church_user_id: church_user.id,
            auth_user_id: church_user.auth_user_id.clone(),
            auth_user,
            created_at: church_user.created_at,
            updated_at: church_user.updated_at,
        })
    }

    /// Find church user by auth user ID
    pub async fn find_by_auth_id(&self, auth_user_id: &str) -> Result<Option<UserModel>> {
        User::find()
            .filter(user::Column::AuthUserId.eq(auth_user_id))
            .one(&self.db)
            .await
            .context("Failed to query church user by auth_user_id")
    }

    /// Find church user by email (queries auth server first)
    pub async fn find_by_email(&self, email: &str) -> Result<Option<CompleteUser>> {
        // Get user from auth server
        let auth_user = self.auth_client.get_user_by_email(email).await?;

        // Find corresponding church user
        let church_user = self.find_by_auth_id(&auth_user.id).await?;

        match church_user {
            Some(cu) => Ok(Some(CompleteUser {
                church_user_id: cu.id,
                auth_user_id: cu.auth_user_id.clone(),
                auth_user,
                created_at: cu.created_at,
                updated_at: cu.updated_at,
            })),
            None => Ok(None),
        }
    }

    /// Link an auth user to the church system
    pub async fn link_auth_user(&self, auth_user_id: &str) -> Result<UserModel> {
        // Verify user exists in auth server
        let _auth_user = self
            .auth_client
            .get_user(auth_user_id)
            .await
            .context("Failed to fetch user from auth server")?;

        // Check if already linked
        if let Some(existing) = self.find_by_auth_id(auth_user_id).await? {
            return Ok(existing);
        }

        // Create new church user record (only stores the link to auth server)
        let now = chrono::Utc::now().naive_utc();
        let new_user = user::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            auth_user_id: Set(auth_user_id.to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let result = new_user
            .insert(&self.db)
            .await
            .context("Failed to create church user record")?;

        Ok(result)
    }

    /// Link auth user by email
    pub async fn link_auth_user_by_email(&self, email: &str) -> Result<UserModel> {
        let auth_user = self
            .auth_client
            .get_user_by_email(email)
            .await
            .context("User not found in auth server")?;

        self.link_auth_user(&auth_user.id).await
    }
}

/// Complete user combining auth and church data
#[derive(Debug, Clone)]
pub struct CompleteUser {
    pub church_user_id: i64,
    pub auth_user_id: String,
    pub auth_user: AuthUser,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl CompleteUser {
    /// Get user's full name
    pub fn full_name(&self) -> String {
        match (&self.auth_user.first_name, &self.auth_user.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.auth_user.email.clone(),
        }
    }

    /// Get user's email
    pub fn email(&self) -> &str {
        &self.auth_user.email
    }

    /// Get user's role
    pub fn role(&self) -> &str {
        &self.auth_user.role
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.auth_user.role.to_lowercase() == "admin"
    }

    /// Check if email is verified
    pub fn is_email_verified(&self) -> bool {
        self.auth_user.email_verified
    }
}
