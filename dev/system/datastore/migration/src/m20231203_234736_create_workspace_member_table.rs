//------------------------------------------------------------------------------
//! Create workspace_member table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::{
    UserAccount,
    Workspace,
    WorkspaceMember,
    WorkspaceMemberAuthority,
};


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
            .table(WorkspaceMember::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(WorkspaceMember::WorkspaceMemberId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(WorkspaceMember::WorkspaceId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(WorkspaceMember::UserAccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(WorkspaceMember::WorkspaceMemberAuthorityId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("workspace_member_workspace_id_fkey")
                    .from(WorkspaceMember::Table, WorkspaceMember::WorkspaceId)
                    .to(Workspace::Table, Workspace::WorkspaceId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("workspace_member_user_account_id_fkey")
                    .from(WorkspaceMember::Table, WorkspaceMember::UserAccountId)
                    .to(UserAccount::Table, UserAccount::UserAccountId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("workspace_member_workspace_member_authority_fkey")
                    .from(WorkspaceMember::Table, WorkspaceMember::WorkspaceMemberAuthorityId)
                    .to(WorkspaceMemberAuthority::Table, WorkspaceMemberAuthority::WorkspaceMemberAuthorityId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("workspace_member_workspace_id_idx")
            .table(WorkspaceMember::Table)
            .col(WorkspaceMember::WorkspaceId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("workspace_member_user_account_id_idx")
            .table(WorkspaceMember::Table)
            .col(WorkspaceMember::UserAccountId)
            .to_owned();
        manager.create_index(index).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(WorkspaceMember::Table).to_owned())
            .await
    }
}
