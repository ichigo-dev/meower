//------------------------------------------------------------------------------
//! Creates account table.
//------------------------------------------------------------------------------

use crate::table_def::Account;

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
            .table(Account::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(Account::AccountId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(Account::AccountName)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(Account::PublicUserId)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Account::DefaultAccountProfileId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Account::DefaultWorkspaceId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Account::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Account::LastLoginAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"account\" IS 'Account table'",
            "COMMENT ON COLUMN \"account\".\"account_id\" IS 'Account ID'",
            "COMMENT ON COLUMN \"account\".\"account_name\" IS 'Account name'",
            "COMMENT ON COLUMN \"account\".\"public_user_id\" IS 'User public ID (auth server user(public_user_id))'",
            "COMMENT ON COLUMN \"account\".\"default_account_profile_id\" IS 'Default account profile ID'",
            "COMMENT ON COLUMN \"account\".\"default_workspace_id\" IS 'Default workspace ID'",
            "COMMENT ON COLUMN \"account\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"account\".\"last_login_at\" IS 'Last login date'",
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
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;

        Ok(())
    }
}
