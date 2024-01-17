//------------------------------------------------------------------------------
//! SeaORM Migration.
//------------------------------------------------------------------------------

mod table_def;
mod migrations;

use migrations::*;
use sea_orm_migration::prelude::*;


//------------------------------------------------------------------------------
/// Migrator.
//------------------------------------------------------------------------------
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
            Box::new(m20240117113356_create_user_table::Migration),
        ]
    }
}
