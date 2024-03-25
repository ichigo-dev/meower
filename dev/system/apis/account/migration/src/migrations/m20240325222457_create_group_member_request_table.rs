//------------------------------------------------------------------------------
//! Creates group_member_request_invitation table.
//------------------------------------------------------------------------------

use crate::table_def::{ Account, AccountProfile, Group, GroupMemberRequest };

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
            .table(GroupMemberRequest::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(GroupMemberRequest::GroupMemberRequestId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(GroupMemberRequest::GroupId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupMemberRequest::AccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupMemberRequest::AccountProfileId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_request_group_id_fkey")
                    .from(GroupMemberRequest::Table, GroupMemberRequest::GroupId)
                    .to(Group::Table, Group::GroupId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_request_account_id_fkey")
                    .from(GroupMemberRequest::Table, GroupMemberRequest::AccountId)
                    .to(Account::Table, Account::AccountId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_request_account_profile_id_fkey")
                    .from(GroupMemberRequest::Table, GroupMemberRequest::AccountProfileId)
                    .to(AccountProfile::Table, AccountProfile::AccountProfileId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("group_member_request_group_id_idx")
            .table(GroupMemberRequest::Table)
            .col(GroupMemberRequest::GroupId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("group_member_request_account_id_idx")
            .table(GroupMemberRequest::Table)
            .col(GroupMemberRequest::AccountId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group_member_request\" IS 'Group member table'",
            "COMMENT ON COLUMN \"group_member_request\".\"group_member_request_id\" IS 'Group member ID'",
            "COMMENT ON COLUMN \"group_member_request\".\"group_id\" IS 'Group ID'",
            "COMMENT ON COLUMN \"group_member_request\".\"account_id\" IS 'Account ID'",
            "COMMENT ON COLUMN \"group_member_request\".\"account_profile_id\" IS 'Account profile ID'",
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
            .drop_table(Table::drop().table(GroupMemberRequest::Table).to_owned())
            .await?;

        Ok(())
    }
}
