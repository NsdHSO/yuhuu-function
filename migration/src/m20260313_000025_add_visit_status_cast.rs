use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create a cast function that converts text to visit_status
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE OR REPLACE FUNCTION church.text_to_visit_status(text) \
                 RETURNS church.visit_status \
                 LANGUAGE sql IMMUTABLE STRICT AS $$ \
                     SELECT $1::church.visit_status; \
                 $$"
            )
            .await?;

        // Create an implicit cast from text to visit_status
        // This allows SeaORM to pass text values that PostgreSQL automatically converts
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE CAST (text AS church.visit_status) \
                 WITH FUNCTION church.text_to_visit_status(text) \
                 AS IMPLICIT"
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the cast
        manager
            .get_connection()
            .execute_unprepared("DROP CAST IF EXISTS (text AS church.visit_status)")
            .await?;

        // Drop the function
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS church.text_to_visit_status(text)")
            .await?;

        Ok(())
    }
}
