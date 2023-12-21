//------------------------------------------------------------------------------
//! Creates organization_workspace table.
//------------------------------------------------------------------------------

use crate::table_def::{ Organization, OrganizationWorkspace, Workspace };

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
            .table(OrganizationWorkspace::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(OrganizationWorkspace::OrganizationWorkspaceId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(OrganizationWorkspace::OrganizationId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(OrganizationWorkspace::WorkspaceId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("organization_workspace_organization_id_fkey")
                    .from(OrganizationWorkspace::Table, OrganizationWorkspace::OrganizationId)
                    .to(Organization::Table, Organization::OrganizationId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("organization_workspace_workspace_id_fkey")
                    .from(OrganizationWorkspace::Table, OrganizationWorkspace::WorkspaceId)
                    .to(Workspace::Table, Workspace::WorkspaceId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("organization_workspace_organization_id_idx")
            .table(OrganizationWorkspace::Table)
            .col(OrganizationWorkspace::OrganizationId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("organization_workspace_workspace_id_idx")
            .table(OrganizationWorkspace::Table)
            .col(OrganizationWorkspace::WorkspaceId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"organization_workspace\" IS 'Organization workspace table'",
            "COMMENT ON COLUMN \"organization_workspace\".\"organization_workspace_id\" IS 'Organization workspace ID'",
            "COMMENT ON COLUMN \"organization_workspace\".\"organization_id\" IS 'Organization ID'",
            "COMMENT ON COLUMN \"organization_workspace\".\"workspace_id\" IS 'Workspace ID'",
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
            .drop_table(Table::drop().table(OrganizationWorkspace::Table).to_owned())
            .await?;

        Ok(())
    }
}
