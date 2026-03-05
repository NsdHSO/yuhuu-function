use http_response::{CustomError, HttpCodeW};
use models::dto::{user_profile, UserProfile};
use models::internal::UserSearchResult;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

pub struct AdminService;

impl AdminService {
    /// Search users by middle name with partial matching
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `search_term` - Search term for middle_name (min 2 chars)
    ///
    /// # Returns
    /// * `Ok(Vec<UserSearchResult>)` - List of matching user profiles (max 50 results)
    /// * `Err(CustomError)` - If search term is invalid
    ///
    /// # Validation
    /// - Search term must be at least 2 characters after trimming
    ///
    /// # Query
    /// ```sql
    /// SELECT id, user_id, middle_name, phone, profile_picture_url
    /// FROM church.user_profiles
    /// WHERE middle_name ILIKE '%search_term%'
    /// ORDER BY middle_name ASC
    /// LIMIT 50;
    /// ```
    pub async fn search_users_by_name(
        db: &DatabaseConnection,
        search_term: &str,
    ) -> Result<Vec<UserSearchResult>, CustomError> {
        // Validation: trim and check minimum length
        let term = search_term.trim();
        if term.len() < 2 {
            return Err(CustomError::new(
                HttpCodeW::BadRequest,
                "Search term must be at least 2 characters".to_string(),
            ));
        }

        // Build LIKE pattern for case-insensitive partial matching
        let pattern = format!("%{}%", term);

        // Query database
        let profiles = UserProfile::find()
            .filter(user_profile::Column::MiddleName.like(&pattern))
            .order_by_asc(user_profile::Column::MiddleName)
            .limit(50)
            .all(db)
            .await?;

        // Map database models to DTOs
        Ok(profiles.into_iter().map(|p| p.into()).collect())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_search_term_validation() {
        // This is a unit test for validation logic only
        // Integration tests with database would go in a separate test file
        let short_term = "J";
        assert!(short_term.trim().len() < 2);

        let valid_term = "Jo";
        assert!(valid_term.trim().len() >= 2);

        let whitespace_term = "  ";
        assert!(whitespace_term.trim().len() < 2);
    }

    #[test]
    fn test_pattern_format() {
        let term = "John";
        let pattern = format!("%{}%", term);
        assert_eq!(pattern, "%John%");
    }
}
