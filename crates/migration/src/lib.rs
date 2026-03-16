pub use sea_orm_migration::prelude::*;

mod m20260316_082539_create_users_table;
mod m20260316_084945_create_mfa_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260316_082539_create_users_table::Migration),
            Box::new(m20260316_084945_create_mfa_table::Migration),
        ]
    }
}
