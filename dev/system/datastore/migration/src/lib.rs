//------------------------------------------------------------------------------
/// SeaORM Migration.
//------------------------------------------------------------------------------

mod m20231127_041811_create_user_table;
mod m20231127_044012_create_account_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator
{
    //--------------------------------------------------------------------------
    /// Migrates database.
    //--------------------------------------------------------------------------
    fn migrations() -> Vec<Box<dyn MigrationTrait>>
    {
        vec!
        [
            Box::new(m20231127_041811_create_user_table::Migration),
            Box::new(m20231127_044012_create_account_table::Migration),
        ]
    }
}
