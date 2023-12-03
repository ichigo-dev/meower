//------------------------------------------------------------------------------
//! Create workspace table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::{ Organization, Workspace };


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
            .table(Workspace::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(Workspace::WorkspaceId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(Workspace::WorkspaceName)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(Workspace::DisplayName)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Workspace::OrganizationId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Workspace::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Workspace::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Workspace::IsDeleted)
                    .boolean()
                    .default(false)
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("workspace_organization_id_fkey")
                    .from(Workspace::Table, Workspace::OrganizationId)
                    .to(Organization::Table, Organization::OrganizationId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("workspace_organization_id_idx")
            .table(Workspace::Table)
            .col(Workspace::OrganizationId)
            .to_owned();
        manager.create_index(index).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await
    }
}
