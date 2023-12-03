//------------------------------------------------------------------------------
//! SeaORM Migration.
//------------------------------------------------------------------------------

mod enum_def;
mod table_def;
mod m20231203_024539_create_user_table;
mod m20231203_033109_create_user_auth_table;
mod m20231203_034916_create_user_account_table;
mod m20231203_040228_create_temporary_user_table;
mod m20231203_040808_create_temporary_user_token_table;
mod m20231203_043102_create_organization_table;
mod m20231203_043802_create_organization_member_table;

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
            Box::new(m20231203_033109_create_user_auth_table::Migration),
            Box::new(m20231203_034916_create_user_account_table::Migration),
            Box::new(m20231203_040228_create_temporary_user_table::Migration),
            Box::new(m20231203_040808_create_temporary_user_token_table::Migration),
            Box::new(m20231203_043102_create_organization_table::Migration),
            Box::new(m20231203_043802_create_organization_member_table::Migration),
        ]
    }
}
