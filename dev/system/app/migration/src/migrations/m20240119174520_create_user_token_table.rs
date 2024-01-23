//------------------------------------------------------------------------------
//! Creates user_token table.
//------------------------------------------------------------------------------

use crate::table_def::UserToken;

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
            .table(UserToken::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserToken::UserTokenId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserToken::Token)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(UserToken::RefreshToken)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserToken::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"user_token\" IS 'User token table'",
            "COMMENT ON COLUMN \"user_token\".\"user_token_id\" IS 'User token ID'",
            "COMMENT ON COLUMN \"user_token\".\"token\" IS 'Token'",
            "COMMENT ON COLUMN \"user_token\".\"refresh_token\" IS 'Refresh token'",
            "COMMENT ON COLUMN \"user_token\".\"created_at\" IS 'Created at'",
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
            .drop_table(Table::drop().table(UserToken::Table).to_owned())
            .await?;

        Ok(())
    }
}
