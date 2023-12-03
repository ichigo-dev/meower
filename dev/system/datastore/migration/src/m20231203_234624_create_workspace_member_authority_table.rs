//------------------------------------------------------------------------------
//! Create workspace_member_authority table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::WorkspaceMemberAuthority;


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
            .table(WorkspaceMemberAuthority::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(WorkspaceMemberAuthority::WorkspaceMemberAuthorityId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(WorkspaceMemberAuthority::Symbol)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(WorkspaceMemberAuthority::Value)
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
            .drop_table(Table::drop().table(WorkspaceMemberAuthority::Table).to_owned())
            .await
    }
}
