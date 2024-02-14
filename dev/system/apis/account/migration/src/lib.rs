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
            Box::new(m20240129124852_create_account_table::Migration),
            Box::new(m20240129134718_create_account_profile_table::Migration),
            Box::new(m20240129135415_create_group_table::Migration),
            Box::new(m20240129145933_create_group_member_table::Migration),
            Box::new(m20240129151907_create_workspace_table::Migration),
            Box::new(m20240129152205_create_account_workspace_table::Migration),
            Box::new(m20240129152701_create_group_workspace_table::Migration),
            Box::new(m20240214185125_create_account_profile_avatar_table::Migration),
        ]
    }
}
