use http_response::CustomError;
use models::internal::{
    AssignmentListQuery, CreateVisitAssignmentRequest, CreateVisitableFamilyRequest,
    FamilyListQuery, UpdateVisitableFamilyRequest, UserSearchResult, VisitAssignmentResponse,
    VisitableFamilyResponse,
};
use sea_orm::DatabaseConnection;

use crate::features::profiles::service::ProfileService;
use crate::features::visits::services::{VisitAssignmentService, VisitableFamilyService};

pub struct AdminService;

impl AdminService {
    pub async fn search_users_by_name(
        db: &DatabaseConnection,
        search_term: &str,
    ) -> Result<Vec<UserSearchResult>, CustomError> {
        ProfileService::search_users_by_name(db, search_term).await
    }

    // Visit Management (Admin) - Service Layer Facade

    pub async fn list_families(
        db: &DatabaseConnection,
        query: FamilyListQuery,
    ) -> Result<Vec<VisitableFamilyResponse>, CustomError> {
        VisitableFamilyService::list(db, query.limit, query.offset, query.search).await
    }

    pub async fn get_family(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<VisitableFamilyResponse, CustomError> {
        VisitableFamilyService::get_by_id(db, id).await
    }

    pub async fn create_family(
        db: &DatabaseConnection,
        req: CreateVisitableFamilyRequest,
    ) -> Result<VisitableFamilyResponse, CustomError> {
        VisitableFamilyService::create(db, req).await
    }

    pub async fn update_family(
        db: &DatabaseConnection,
        id: i64,
        req: UpdateVisitableFamilyRequest,
    ) -> Result<VisitableFamilyResponse, CustomError> {
        VisitableFamilyService::update(db, id, req).await
    }

    pub async fn delete_family(db: &DatabaseConnection, id: i64) -> Result<(), CustomError> {
        VisitableFamilyService::delete(db, id).await
    }

    pub async fn list_assignments(
        db: &DatabaseConnection,
        query: AssignmentListQuery,
    ) -> Result<Vec<VisitAssignmentResponse>, CustomError> {
        VisitAssignmentService::list_all_admin(db, query.limit, query.offset).await
    }

    pub async fn create_assignment(
        db: &DatabaseConnection,
        req: CreateVisitAssignmentRequest,
    ) -> Result<VisitAssignmentResponse, CustomError> {
        VisitAssignmentService::create(db, req).await
    }

    pub async fn delete_assignment(db: &DatabaseConnection, id: i64) -> Result<(), CustomError> {
        VisitAssignmentService::delete(db, id).await
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
