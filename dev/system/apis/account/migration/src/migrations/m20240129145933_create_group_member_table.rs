//------------------------------------------------------------------------------
//! Creates group_member table.
//------------------------------------------------------------------------------

use crate::table_def::{ Account, AccountProfile, Group, GroupMember };

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
            .table(GroupMember::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(GroupMember::GroupMemberId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(GroupMember::GroupId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupMember::AccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupMember::AccountProfileId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupMember::Role)
                    .tiny_integer()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_group_id_fkey")
                    .from(GroupMember::Table, GroupMember::GroupId)
                    .to(Group::Table, Group::GroupId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_account_id_fkey")
                    .from(GroupMember::Table, GroupMember::AccountId)
                    .to(Account::Table, Account::AccountId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_member_account_profile_id_fkey")
                    .from(GroupMember::Table, GroupMember::AccountProfileId)
                    .to(AccountProfile::Table, AccountProfile::AccountProfileId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("group_member_group_id_idx")
            .table(GroupMember::Table)
            .col(GroupMember::GroupId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("group_member_account_id_idx")
            .table(GroupMember::Table)
            .col(GroupMember::AccountId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group_member\" IS 'Group member table'",
            "COMMENT ON COLUMN \"group_member\".\"group_member_id\" IS 'Group member ID'",
            "COMMENT ON COLUMN \"group_member\".\"group_id\" IS 'Group ID'",
            "COMMENT ON COLUMN \"group_member\".\"account_id\" IS 'Account ID'",
            "COMMENT ON COLUMN \"group_member\".\"account_profile_id\" IS 'Account profile ID'",
            "COMMENT ON COLUMN \"group_member\".\"role\" IS 'Role'",
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
            .drop_table(Table::drop().table(GroupMember::Table).to_owned())
            .await?;

        Ok(())
    }
}
