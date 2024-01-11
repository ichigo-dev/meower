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
            Box::new(m20231203_024539_create_user_table::Migration),
            Box::new(m20231203_033109_create_user_auth_table::Migration),
            Box::new(m20231203_034916_create_user_account_table::Migration),
            Box::new(m20231203_040228_create_temporary_user_table::Migration),
            Box::new(m20231203_040808_create_temporary_user_code_table::Migration),
            Box::new(m20231203_043102_create_organization_table::Migration),
            Box::new(m20231203_043802_create_organization_member_authority_table::Migration),
            Box::new(m20231203_231817_create_organization_member_table::Migration),
            Box::new(m20231203_233644_create_workspace_table::Migration),
            Box::new(m20231203_234624_create_workspace_member_authority_table::Migration),
            Box::new(m20231203_234736_create_workspace_member_table::Migration),
            Box::new(m20231204_034523_create_project_table::Migration),
            Box::new(m20231204_040001_create_project_member_authority_table::Migration),
            Box::new(m20231204_062533_create_project_member_table::Migration),
            Box::new(m20231204_062921_create_task_table::Migration),
            Box::new(m20231204_080345_create_task_member_table::Migration),
            Box::new(m20231210_023700_create_reset_password_token_table::Migration),
            Box::new(m20231210_134129_create_user_jwt_subject_table::Migration),
            Box::new(m20231213_054703_create_user_account_profile_table::Migration),
            Box::new(m20231216_134816_create_user_account_workspace_table::Migration),
            Box::new(m20231216_135419_create_organization_workspace_table::Migration),
            Box::new(m20240111_090712_create_user_jwt_refresh_token_table::Migration),
        ]
    }
}
