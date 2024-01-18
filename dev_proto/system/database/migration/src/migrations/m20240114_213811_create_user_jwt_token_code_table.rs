//------------------------------------------------------------------------------
//! Creates user_jwt_token_code table.
//------------------------------------------------------------------------------

use crate::table_def::UserJwtTokenCode;

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
            .table(UserJwtTokenCode::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserJwtTokenCode::UserJwtTokenCodeId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserJwtTokenCode::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserJwtTokenCode::Code)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(UserJwtTokenCode::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserJwtTokenCode::ExpiredAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"user_jwt_token_code\" IS 'User JWT token code'",
            "COMMENT ON COLUMN \"user_jwt_token_code\".\"user_jwt_token_code_id\" IS 'User JWT token code ID'",
            "COMMENT ON COLUMN \"user_jwt_token_code\".\"user_id\" IS 'User ID'",
            "COMMENT ON COLUMN \"user_jwt_token_code\".\"code\" IS 'Code'",
            "COMMENT ON COLUMN \"user_jwt_token_code\".\"created_at\" IS 'Created at'",
            "COMMENT ON COLUMN \"user_jwt_token_code\".\"expired_at\" IS 'Expired at'",
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
            .drop_table(Table::drop().table(UserJwtTokenCode::Table).to_owned())
            .await?;

        Ok(())
    }
}
