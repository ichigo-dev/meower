//------------------------------------------------------------------------------
//! Creates workspace table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use crate::table_def::Workspace;


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
            .to_owned();
        manager.create_table(table).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"workspace\" IS 'Workspace table'",
            "COMMENT ON COLUMN \"workspace\".\"workspace_id\" IS 'Workspace ID'",
            "COMMENT ON COLUMN \"workspace\".\"workspace_name\" IS 'Workspace name'",
            "COMMENT ON COLUMN \"workspace\".\"display_name\" IS 'Display name'",
            "COMMENT ON COLUMN \"workspace\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"workspace\".\"updated_at\" IS 'Update date'",
            "COMMENT ON COLUMN \"workspace\".\"is_deleted\" IS 'Soft delete flag'",
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
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await?;

        Ok(())
    }
}
