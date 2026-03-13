use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), VisitAssignments::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VisitAssignments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VisitAssignments::FamilyId).big_integer().not_null())
                    .col(ColumnDef::new(VisitAssignments::AssignedToUserId).big_integer().not_null())
                    .col(ColumnDef::new(VisitAssignments::ScheduledDate).date().not_null())
                    .col(ColumnDef::new(VisitAssignments::ArrivedAt).timestamp())
                    .col(ColumnDef::new(VisitAssignments::ArrivedLatitude).decimal_len(10, 8))
                    .col(ColumnDef::new(VisitAssignments::ArrivedLongitude).decimal_len(11, 8))
                    .col(ColumnDef::new(VisitAssignments::CompletedAt).timestamp())
                    .col(ColumnDef::new(VisitAssignments::Notes).text())
                    .col(
                        ColumnDef::new(VisitAssignments::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(VisitAssignments::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_visit_assignments_family_id")
                            .from(
                                (Alias::new("church"), VisitAssignments::Table),
                                VisitAssignments::FamilyId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("visitable_families")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_visit_assignments_user_id")
                            .from(
                                (Alias::new("church"), VisitAssignments::Table),
                                VisitAssignments::AssignedToUserId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.visit_assignments ADD COLUMN status church.visit_status NOT NULL DEFAULT 'pending'"
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_visit_assignments_user")
                    .table((Alias::new("church"), VisitAssignments::Table))
                    .col(VisitAssignments::AssignedToUserId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_visit_assignments_family")
                    .table((Alias::new("church"), VisitAssignments::Table))
                    .col(VisitAssignments::FamilyId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_visit_assignments_date")
                    .table((Alias::new("church"), VisitAssignments::Table))
                    .col(VisitAssignments::ScheduledDate)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE church.visit_assignments ADD CONSTRAINT chk_arrived_before_completed CHECK (arrived_at < completed_at OR completed_at IS NULL)"
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), VisitAssignments::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum VisitAssignments {
    Table,
    Id,
    FamilyId,
    AssignedToUserId,
    ScheduledDate,
    ArrivedAt,
    ArrivedLatitude,
    ArrivedLongitude,
    CompletedAt,
    Notes,
    CreatedAt,
    UpdatedAt,
}
