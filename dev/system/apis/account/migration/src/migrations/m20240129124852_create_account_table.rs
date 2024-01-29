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
                ColumnDef::new(Account::UserSubject)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(Account::CreatedAt)
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
            "COMMENT ON COLUMN \"account\".\"user_subject\" IS 'User subject'",
            "COMMENT ON COLUMN \"account\".\"created_at\" IS 'Create date'",
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
