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
            Box::new(m20240117122705_create_user_auth_table::Migration),
            Box::new(m20240117122852_create_temporary_user_table::Migration),
            Box::new(m20240117123226_create_temporary_user_code_table::Migration),
            Box::new(m20240117123622_create_reset_password_token_table::Migration),
            Box::new(m20240117124025_create_jwt_token_code_table::Migration),
            Box::new(m20240117124345_create_jwt_refresh_token_table::Migration),
            Box::new(m20240117124707_create_client_application_table::Migration),
            Box::new(m20240126170310_create_client_application_allow_origin_table::Migration),
        ]
    }
}
