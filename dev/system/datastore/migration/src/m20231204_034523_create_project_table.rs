//------------------------------------------------------------------------------
//! Creates project table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
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
        // Creates a table.
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

        // Creates indexes.
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
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"project\" IS 'Project table'",
            "COMMENT ON COLUMN \"project\".\"project_id\" IS 'Project ID'",
            "COMMENT ON COLUMN \"project\".\"project_name\" IS 'Project name'",
            "COMMENT ON COLUMN \"project\".\"display_name\" IS 'Display name'",
            "COMMENT ON COLUMN \"project\".\"workspace_id\" IS 'Workspace ID'",
            "COMMENT ON COLUMN \"project\".\"created_at\" IS 'Created date'",
            "COMMENT ON COLUMN \"project\".\"updated_at\" IS 'Updated date'",
            "COMMENT ON COLUMN \"project\".\"is_deleted\" IS 'Soft delete flag'",
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
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;

        Ok(())
    }
}
