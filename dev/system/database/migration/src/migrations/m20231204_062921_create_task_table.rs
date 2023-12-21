//------------------------------------------------------------------------------
//! Creates task table.
//------------------------------------------------------------------------------

use crate::table_def::{ UserAccount, Task, Project };

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
            .table(Task::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(Task::TaskId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(Task::ProjectId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Task::OwnerUserAccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Task::Title)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Task::Content)
                    .text()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Task::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Task::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Task::IsDeleted)
                    .boolean()
                    .default(false)
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("task_project_id_fkey")
                    .from(Task::Table, Task::ProjectId)
                    .to(Project::Table, Project::ProjectId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("task_owner_user_account_id_fkey")
                    .from(Task::Table, Task::OwnerUserAccountId)
                    .to(UserAccount::Table, UserAccount::UserAccountId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("task_project_id_idx")
            .table(Task::Table)
            .col(Task::ProjectId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("task_owner_user_account_id_idx")
            .table(Task::Table)
            .col(Task::OwnerUserAccountId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"task\" IS 'Task table'",
            "COMMENT ON COLUMN \"task\".\"task_id\" IS 'Task ID'",
            "COMMENT ON COLUMN \"task\".\"project_id\" IS 'Project ID'",
            "COMMENT ON COLUMN \"task\".\"owner_user_account_id\" IS 'Owner user account ID'",
            "COMMENT ON COLUMN \"task\".\"title\" IS 'Title'",
            "COMMENT ON COLUMN \"task\".\"content\" IS 'Content'",
            "COMMENT ON COLUMN \"task\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"task\".\"updated_at\" IS 'Update date'",
            "COMMENT ON COLUMN \"task\".\"is_deleted\" IS 'Soft delete flag'",
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
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;

        Ok(())
    }
}
