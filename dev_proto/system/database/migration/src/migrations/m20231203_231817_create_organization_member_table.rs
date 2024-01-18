//------------------------------------------------------------------------------
//! Creates organization_member table.
//------------------------------------------------------------------------------

use crate::table_def::{
    UserAccount,
    Organization,
    OrganizationMember,
    OrganizationMemberAuthority,
};

use sea_orm_migration::prelude::*;
use sea_orm::Statement;


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
        // Creates a table.
        let table = Table::create()
            .table(OrganizationMember::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(OrganizationMember::OrganizationMemberId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(OrganizationMember::OrganizationId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(OrganizationMember::UserAccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(OrganizationMember::OrganizationMemberAuthorityId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("organization_member_organization_id_fkey")
                    .from(OrganizationMember::Table, OrganizationMember::OrganizationId)
                    .to(Organization::Table, Organization::OrganizationId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("organization_member_user_account_id_fkey")
                    .from(OrganizationMember::Table, OrganizationMember::UserAccountId)
                    .to(UserAccount::Table, UserAccount::UserAccountId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("organization_member_organization_member_authority_fkey")
                    .from(OrganizationMember::Table, OrganizationMember::OrganizationMemberAuthorityId)
                    .to(OrganizationMemberAuthority::Table, OrganizationMemberAuthority::OrganizationMemberAuthorityId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("organization_member_organization_id_idx")
            .table(OrganizationMember::Table)
            .col(OrganizationMember::OrganizationId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("organization_member_user_account_id_idx")
            .table(OrganizationMember::Table)
            .col(OrganizationMember::UserAccountId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"organization_member\" IS 'Organization member table'",
            "COMMENT ON COLUMN \"organization_member\".\"organization_member_id\" IS 'Organization member ID'",
            "COMMENT ON COLUMN \"organization_member\".\"organization_id\" IS 'Organization ID'",
            "COMMENT ON COLUMN \"organization_member\".\"user_account_id\" IS 'User account ID'",
            "COMMENT ON COLUMN \"organization_member\".\"organization_member_authority_id\" IS 'Organization member authority ID'",
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
        // Drops a table.
        manager
            .drop_table(Table::drop().table(OrganizationMember::Table).to_owned())
            .await?;

        Ok(())
    }
}
