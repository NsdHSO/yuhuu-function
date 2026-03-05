use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Use raw SQL with IF NOT EXISTS to handle cases where columns may already exist
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.user_profiles \
                 ADD COLUMN IF NOT EXISTS education_level VARCHAR"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.user_profiles \
                 ADD COLUMN IF NOT EXISTS field_of_study VARCHAR"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.user_profiles \
                 ADD COLUMN IF NOT EXISTS languages_spoken VARCHAR"
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Use raw SQL with IF EXISTS to handle rollback safely
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.user_profiles \
                 DROP COLUMN IF EXISTS languages_spoken"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.user_profiles \
                 DROP COLUMN IF EXISTS field_of_study"
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.user_profiles \
                 DROP COLUMN IF EXISTS education_level"
            )
            .await?;

        Ok(())
    }
}
