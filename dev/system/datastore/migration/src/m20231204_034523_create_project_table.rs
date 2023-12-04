//------------------------------------------------------------------------------
//! Create project table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::{ Workspace, Project };


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
            .table(Project::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(Project::ProjectId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(Project::ProjectName)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Project::DisplayName)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Project::WorkspaceId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Project::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Project::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Project::IsDeleted)
                    .boolean()
                    .default(false)
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("project_workspace_id_fkey")
                    .from(Project::Table, Project::WorkspaceId)
                    .to(Workspace::Table, Workspace::WorkspaceId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("project_project_name_idx")
            .table(Project::Table)
            .col(Project::ProjectName)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("project_workspace_id_idx")
            .table(Project::Table)
            .col(Project::WorkspaceId)
            .to_owned();
        manager.create_index(index).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}
