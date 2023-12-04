//------------------------------------------------------------------------------
//! Create project_member_authority table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::ProjectMemberAuthority;


//------------------------------------------------------------------------------
/// Migration.
//------------------------------------------------------------------------------
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    //--------------------------------------------------------------------------
    /// Up.
    //--------------------------------------------------------------------------
    async fn up( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        let table = Table::create()
            .table(ProjectMemberAuthority::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(ProjectMemberAuthority::ProjectMemberAuthorityId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(ProjectMemberAuthority::Symbol)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ProjectMemberAuthority::Value)
                    .integer()
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(ProjectMemberAuthority::Table).to_owned())
            .await
    }
}
