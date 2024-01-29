//------------------------------------------------------------------------------
//! Creates account_workspace table.
//------------------------------------------------------------------------------

use crate::table_def::{ Account, AccountWorkspace, Workspace };

use sea_orm::Statement;
use sea_orm_migration::prelude::*;


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
            .table(AccountWorkspace::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(AccountWorkspace::AccountWorkspaceId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(AccountWorkspace::WorkspaceId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountWorkspace::AccountId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("account_workspace_workspace_id_fk")
                    .from(AccountWorkspace::Table, AccountWorkspace::WorkspaceId)
                    .to(Workspace::Table, Workspace::WorkspaceId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("account_workspace_account_id_fk")
                    .from(AccountWorkspace::Table, AccountWorkspace::AccountId)
                    .to(Account::Table, Account::AccountId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("account_workspace_workspace_id_idx")
            .table(AccountWorkspace::Table)
            .col(AccountWorkspace::WorkspaceId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("account_workspace_account_id_idx")
            .table(AccountWorkspace::Table)
            .col(AccountWorkspace::AccountId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"account_workspace\" IS 'Account workspace table'",
            "COMMENT ON COLUMN \"account_workspace\".\"account_workspace_id\" IS 'Account workspace ID'",
            "COMMENT ON COLUMN \"account_workspace\".\"workspace_id\" IS 'Workspace ID'",
            "COMMENT ON COLUMN \"account_workspace\".\"account_id\" IS 'Account ID'",
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
        manager
            .drop_table(Table::drop().table(AccountWorkspace::Table).to_owned())
            .await?;

        Ok(())
    }
}
