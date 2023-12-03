//------------------------------------------------------------------------------
//! Create organization_member table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::enum_def::OrganizationMemberAuthority;
use crate::table_def::{ UserAccount, Organization, OrganizationMember };
use sea_orm_migration::prelude::extension::postgres::Type;


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
        manager
            .create_type
            (
                Type::create()
                    .as_enum(OrganizationMemberAuthority::Table)
                    .values(
                    [
                        OrganizationMemberAuthority::Member,
                        OrganizationMemberAuthority::Admin,
                    ])
                    .to_owned()
            )
            .await?;

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
                ColumnDef::new(OrganizationMember::Authority)
                    .enumeration
                    (
                        OrganizationMemberAuthority::Table,
                        [
                            OrganizationMemberAuthority::Member,
                            OrganizationMemberAuthority::Admin,
                        ]
                    )
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
            .to_owned();
        manager.create_table(table).await?;

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
        manager.create_index(index).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(OrganizationMember::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(OrganizationMemberAuthority::Table).to_owned())
            .await
    }
}
