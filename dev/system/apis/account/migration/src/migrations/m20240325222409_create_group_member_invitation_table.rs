//------------------------------------------------------------------------------
//! Creates group_member_invitation table.
//------------------------------------------------------------------------------

use crate::table_def::{ Account, Group, GroupMemberInvitation };

use sea_orm::Statement;
use sea_orm_migration::prelude::*;


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
            .table(GroupMemberInvitation::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(GroupMemberInvitation::GroupMemberInvitationId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(GroupMemberInvitation::GroupId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupMemberInvitation::AccountId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_invitation_group_id_fkey")
                    .from(GroupMemberInvitation::Table, GroupMemberInvitation::GroupId)
                    .to(Group::Table, Group::GroupId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_invitation_account_id_fkey")
                    .from(GroupMemberInvitation::Table, GroupMemberInvitation::AccountId)
                    .to(Account::Table, Account::AccountId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("group_member_invitation_group_id_idx")
            .table(GroupMemberInvitation::Table)
            .col(GroupMemberInvitation::GroupId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("group_member_invitation_account_id_idx")
            .table(GroupMemberInvitation::Table)
            .col(GroupMemberInvitation::AccountId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group_member_invitation\" IS 'Group member table'",
            "COMMENT ON COLUMN \"group_member_invitation\".\"group_member_invitation_id\" IS 'Group member ID'",
            "COMMENT ON COLUMN \"group_member_invitation\".\"group_id\" IS 'Group ID'",
            "COMMENT ON COLUMN \"group_member_invitation\".\"account_id\" IS 'Account ID'",
        ];
        let hdb = manager.get_connection();
        let backend = manager.get_database_backend();
        for query in querys
        {
            hdb.execute(Statement::from_string(backend, query)).await?;
        }

        Ok(())
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(GroupMemberInvitation::Table).to_owned())
            .await?;

        Ok(())
    }
}
