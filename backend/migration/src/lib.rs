pub use sea_orm_migration::prelude::*;

mod entity;
mod m20231015_180325_create_users_table;
mod m20231015_183517_create_accounts_table;
mod m20231015_192911_create_transactions_table;
mod m20231015_194732_seed_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231015_180325_create_users_table::Migration),
            Box::new(m20231015_183517_create_accounts_table::Migration),
            Box::new(m20231015_192911_create_transactions_table::Migration),
            Box::new(m20231015_194732_seed_data::Migration),
        ]
    }
}
