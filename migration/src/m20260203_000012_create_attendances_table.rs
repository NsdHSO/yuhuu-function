use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), Attendances::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attendances::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Attendances::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Attendances::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Attendances::ServiceType).string().not_null())
                    .col(
                        ColumnDef::new(Attendances::AttendanceDate)
                            .date()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Attendances::CheckInTime).timestamp())
                    .col(ColumnDef::new(Attendances::CheckOutTime).timestamp())
                    .col(
                        ColumnDef::new(Attendances::Status)
                            .string()
                            .not_null()
                            .default("Present"),
                    )
                    .col(ColumnDef::new(Attendances::Notes).text())
                    .col(ColumnDef::new(Attendances::RecordedBy).big_integer())
                    .col(
                        ColumnDef::new(Attendances::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Attendances::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendances_user_id")
                            .from(
                                (Alias::new("church"), Attendances::Table),
                                Attendances::UserId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_attendances_recorded_by")
                            .from(
                                (Alias::new("church"), Attendances::Table),
                                Attendances::RecordedBy,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("users")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index for faster lookups by date and service type
        manager
            .create_index(
                Index::create()
                    .name("idx_attendances_date_service")
                    .table((Alias::new("church"), Attendances::Table))
                    .col(Attendances::AttendanceDate)
                    .col(Attendances::ServiceType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), Attendances::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Attendances {
    Table,
    Id,
    Uuid,
    UserId,
    ServiceType,
    AttendanceDate,
    CheckInTime,
    CheckOutTime,
    Status,
    Notes,
    RecordedBy,
    CreatedAt,
    UpdatedAt,
}
