use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), UserMinistries::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserMinistries::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserMinistries::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserMinistries::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserMinistries::MinistryId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserMinistries::Position).string())
                    .col(ColumnDef::new(UserMinistries::JoinDate).date().not_null())
                    .col(
                        ColumnDef::new(UserMinistries::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(UserMinistries::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserMinistries::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_ministries_user_id")
                            .from(
                                (Alias::new("church"), UserMinistries::Table),
                                UserMinistries::UserId,
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
                            .name("fk_user_ministries_ministry_id")
                            .from(
                                (Alias::new("church"), UserMinistries::Table),
                                UserMinistries::MinistryId,
                            )
                            .to(
                                (Alias::new("church"), Alias::new("ministries")),
                                Alias::new("id"),
                            )
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint: one user can join the same ministry only once
        manager
            .create_index(
                Index::create()
                    .name("idx_user_ministries_user_ministry_unique")
                    .table((Alias::new("church"), UserMinistries::Table))
                    .col(UserMinistries::UserId)
                    .col(UserMinistries::MinistryId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new("church"), UserMinistries::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserMinistries {
    Table,
    Id,
    Uuid,
    UserId,
    MinistryId,
    Position,
    JoinDate,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
