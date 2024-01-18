//------------------------------------------------------------------------------
//! Creates user_account_workspace table.
//------------------------------------------------------------------------------

use crate::table_def::{ UserAccount, UserAccountWorkspace, Workspace };

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
            .table(UserAccountWorkspace::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserAccountWorkspace::UserAccountWorkspaceId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserAccountWorkspace::UserAccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccountWorkspace::WorkspaceId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("user_account_workspace_user_account_id_fkey")
                    .from(UserAccountWorkspace::Table, UserAccountWorkspace::UserAccountId)
                    .to(UserAccount::Table, UserAccount::UserAccountId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("user_account_workspace_workspace_id_fkey")
                    .from(UserAccountWorkspace::Table, UserAccountWorkspace::WorkspaceId)
                    .to(Workspace::Table, Workspace::WorkspaceId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("user_account_workspace_user_account_id_idx")
            .table(UserAccountWorkspace::Table)
            .col(UserAccountWorkspace::UserAccountId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("user_account_workspace_workspace_id_idx")
            .table(UserAccountWorkspace::Table)
            .col(UserAccountWorkspace::WorkspaceId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"user_account_workspace\" IS 'User account workspace table'",
            "COMMENT ON COLUMN \"user_account_workspace\".\"user_account_workspace_id\" IS 'User account workspace ID'",
            "COMMENT ON COLUMN \"user_account_workspace\".\"user_account_id\" IS 'User account ID'",
            "COMMENT ON COLUMN \"user_account_workspace\".\"workspace_id\" IS 'Workspace ID'",
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
            .drop_table(Table::drop().table(UserAccountWorkspace::Table).to_owned())
            .await?;

        Ok(())
    }
}
