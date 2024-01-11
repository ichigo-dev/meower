//------------------------------------------------------------------------------
//! Creates user_jwt_refresh_token table.
//------------------------------------------------------------------------------

use crate::table_def::UserJwtRefreshToken;

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
            .table(UserJwtRefreshToken::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserJwtRefreshToken::UserJwtRefreshTokenId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserJwtRefreshToken::Token)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(UserJwtRefreshToken::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserJwtRefreshToken::ExpiredAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"user_jwt_refresh_token\" IS 'User JWT refresh token'",
            "COMMENT ON COLUMN \"user_jwt_refresh_token\".\"user_jwt_refresh_token_id\" IS 'User JWT refresh token ID'",
            "COMMENT ON COLUMN \"user_jwt_refresh_token\".\"token\" IS 'Token'",
            "COMMENT ON COLUMN \"user_jwt_refresh_token\".\"created_at\" IS 'Created at'",
            "COMMENT ON COLUMN \"user_jwt_refresh_token\".\"expired_at\" IS 'Expired at'",
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
            .drop_table(Table::drop().table(UserJwtRefreshToken::Table).to_owned())
            .await?;

        Ok(())
    }
}
