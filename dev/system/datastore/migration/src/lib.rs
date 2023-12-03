//------------------------------------------------------------------------------
//! SeaORM Migration.
//------------------------------------------------------------------------------

mod m20231203_024539_create_user_table;

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
            Box::new(m20231203_024539_create_user_table::Migration),
        ]
    }
}
