use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create church schema
        manager
            .get_connection()
            .execute_unprepared("CREATE SCHEMA IF NOT EXISTS church")
            .await?;

        // Create migration tracking table in church schema
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS church.seaorm_migration (
                    version VARCHAR NOT NULL PRIMARY KEY,
                    applied_at BIGINT NOT NULL
                )",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop church schema
        manager
            .get_connection()
            .execute_unprepared("DROP SCHEMA IF EXISTS church CASCADE")
            .await?;

        Ok(())
    }
}
