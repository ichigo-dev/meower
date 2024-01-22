//------------------------------------------------------------------------------
//! Creates jwt_refresh_token table.
//------------------------------------------------------------------------------

use crate::table_def::{ JwtRefreshToken, User };

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
            .table(JwtRefreshToken::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(JwtRefreshToken::JwtRefreshTokenId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(JwtRefreshToken::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(JwtRefreshToken::Token)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(JwtRefreshToken::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(JwtRefreshToken::ExpiredAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("jwt_refresh_token_user_id_fkey")
                    .from(JwtRefreshToken::Table, JwtRefreshToken::UserId)
                    .to(User::Table, User::UserId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("jwt_refresh_token_user_id_idx")
            .table(JwtRefreshToken::Table)
            .col(JwtRefreshToken::UserId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"jwt_refresh_token\" IS 'JWT refresh token table';",
            "COMMENT ON COLUMN \"jwt_refresh_token\".\"jwt_refresh_token_id\" IS 'JWT refresh token ID';",
            "COMMENT ON COLUMN \"jwt_refresh_token\".\"user_id\" IS 'User ID';",
            "COMMENT ON COLUMN \"jwt_refresh_token\".\"token\" IS 'Token';",
            "COMMENT ON COLUMN \"jwt_refresh_token\".\"created_at\" IS 'Create date';",
            "COMMENT ON COLUMN \"jwt_refresh_token\".\"expired_at\" IS 'Expire date';",
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
            .drop_table(Table::drop().table(JwtRefreshToken::Table).to_owned())
            .await?;

        Ok(())
    }
}
