//------------------------------------------------------------------------------
//! Creates organization_member_authority table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use crate::table_def::OrganizationMemberAuthority;


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
            .table(OrganizationMemberAuthority::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(OrganizationMemberAuthority::OrganizationMemberAuthorityId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(OrganizationMemberAuthority::Symbol)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(OrganizationMemberAuthority::Value)
                    .integer()
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"organization_member_authority\" IS 'Organization member authority';",
            "COMMENT ON COLUMN \"organization_member_authority\".\"organization_member_authority_id\" IS 'Organization member authority ID';",
            "COMMENT ON COLUMN \"organization_member_authority\".\"symbol\" IS 'Symbol';",
            "COMMENT ON COLUMN \"organization_member_authority\".\"value\" IS 'Value';",
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
            .drop_table(Table::drop().table(OrganizationMemberAuthority::Table).to_owned())
            .await?;

        Ok(())
    }
}
