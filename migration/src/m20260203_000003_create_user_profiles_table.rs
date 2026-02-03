use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new("church"), UserProfiles::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProfiles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserProfiles::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(UserProfiles::UserId).big_integer().not_null().unique_key())
                    .col(ColumnDef::new(UserProfiles::MiddleName).string())
                    .col(ColumnDef::new(UserProfiles::Phone).string())
                    .col(ColumnDef::new(UserProfiles::PhoneSecondary).string())
                    .col(ColumnDef::new(UserProfiles::DateOfBirth).date())
                    .col(ColumnDef::new(UserProfiles::Gender).string())
                    .col(ColumnDef::new(UserProfiles::MaritalStatus).string())
                    .col(ColumnDef::new(UserProfiles::Occupation).string())
                    .col(ColumnDef::new(UserProfiles::Nationality).string())
                    .col(ColumnDef::new(UserProfiles::EmergencyContactName).string())
                    .col(ColumnDef::new(UserProfiles::EmergencyContactPhone).string())
                    .col(ColumnDef::new(UserProfiles::EmergencyContactRelationship).string())
                    .col(ColumnDef::new(UserProfiles::ProfilePictureUrl).string())
                    .col(ColumnDef::new(UserProfiles::Bio).text())
                    .col(ColumnDef::new(UserProfiles::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(UserProfiles::UpdatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_profiles_user_id")
                            .from((Alias::new("church"), UserProfiles::Table), UserProfiles::UserId)
                            .to((Alias::new("church"), Alias::new("users")), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table((Alias::new("church"), UserProfiles::Table)).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserProfiles {
    Table,
    Id,
    Uuid,
    UserId,
    MiddleName,
    Phone,
    PhoneSecondary,
    DateOfBirth,
    Gender,
    MaritalStatus,
    Occupation,
    Nationality,
    EmergencyContactName,
    EmergencyContactPhone,
    EmergencyContactRelationship,
    ProfilePictureUrl,
    Bio,
    CreatedAt,
    UpdatedAt,
}
