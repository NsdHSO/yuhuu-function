pub use sea_orm_migration::prelude::*;

mod m20260203_000001_create_church_schema;
mod m20260203_000002_create_users_table;
mod m20260203_000003_create_user_profiles_table;
mod m20260203_000004_create_user_addresses_table;
mod m20260203_000005_create_zones_table;
mod m20260203_000006_create_cell_groups_table;
mod m20260203_000007_create_roles_table;
mod m20260203_000008_create_ministries_table;
mod m20260203_000009_create_user_memberships_table;
mod m20260203_000010_create_user_roles_table;
mod m20260203_000011_create_user_ministries_table;
mod m20260203_000012_create_attendances_table;
mod m20260203_000013_create_givings_table;
mod m20260203_000014_seed_default_roles;
mod m20260203_000015_create_dinners_table;
mod m20260203_000016_create_dinner_participants_table;
mod m20260203_000017_create_family_relationships_table;
mod m20260203_000018_create_spiritual_milestones_table;
mod m20260203_000019_create_membership_history_table;
mod m20260203_000020_create_user_skills_table;
mod m20260203_000021_alter_user_profiles_add_education_fields;
mod m20260312_000022_create_visit_status_enum;
mod m20260312_000023_create_visitable_families_table;
mod m20260312_000024_create_visit_assignments_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260203_000001_create_church_schema::Migration),
            Box::new(m20260203_000002_create_users_table::Migration),
            Box::new(m20260203_000003_create_user_profiles_table::Migration),
            Box::new(m20260203_000004_create_user_addresses_table::Migration),
            Box::new(m20260203_000005_create_zones_table::Migration),
            Box::new(m20260203_000006_create_cell_groups_table::Migration),
            Box::new(m20260203_000007_create_roles_table::Migration),
            Box::new(m20260203_000008_create_ministries_table::Migration),
            Box::new(m20260203_000009_create_user_memberships_table::Migration),
            Box::new(m20260203_000010_create_user_roles_table::Migration),
            Box::new(m20260203_000011_create_user_ministries_table::Migration),
            Box::new(m20260203_000012_create_attendances_table::Migration),
            Box::new(m20260203_000013_create_givings_table::Migration),
            Box::new(m20260203_000014_seed_default_roles::Migration),
            Box::new(m20260203_000015_create_dinners_table::Migration),
            Box::new(m20260203_000016_create_dinner_participants_table::Migration),
            Box::new(m20260203_000017_create_family_relationships_table::Migration),
            Box::new(m20260203_000018_create_spiritual_milestones_table::Migration),
            Box::new(m20260203_000019_create_membership_history_table::Migration),
            Box::new(m20260203_000020_create_user_skills_table::Migration),
            Box::new(m20260203_000021_alter_user_profiles_add_education_fields::Migration),
            Box::new(m20260312_000022_create_visit_status_enum::Migration),
            Box::new(m20260312_000023_create_visitable_families_table::Migration),
            Box::new(m20260312_000024_create_visit_assignments_table::Migration),
        ]
    }

    // Override to store seaql_migrations table in church schema
    fn migration_table_name() -> DynIden {
        Alias::new("church.seaql_migrations").into_iden()
    }
}
