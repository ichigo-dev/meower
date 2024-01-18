//------------------------------------------------------------------------------
//! Creates jwt_token_code table.
//------------------------------------------------------------------------------

use crate::table_def::{ JwtTokenCode, User };

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
            .table(JwtTokenCode::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(JwtTokenCode::JwtTokenCodeId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(JwtTokenCode::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(JwtTokenCode::Code)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(JwtTokenCode::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(JwtTokenCode::ExpiredAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("jwt_token_code_user_id_fkey")
                    .from(JwtTokenCode::Table, JwtTokenCode::UserId)
                    .to(User::Table, User::UserId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("jwt_token_code_user_id_idx")
            .table(JwtTokenCode::Table)
            .col(JwtTokenCode::UserId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"jwt_token_code\" IS 'JWT token code table';",
            "COMMENT ON COLUMN \"jwt_token_code\".\"jwt_token_code_id\" IS 'JWT token code ID';",
            "COMMENT ON COLUMN \"jwt_token_code\".\"user_id\" IS 'User ID';",
            "COMMENT ON COLUMN \"jwt_token_code\".\"code\" IS 'Code';",
            "COMMENT ON COLUMN \"jwt_token_code\".\"created_at\" IS 'Create date';",
            "COMMENT ON COLUMN \"jwt_token_code\".\"expired_at\" IS 'Expire date';",
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
            .drop_table(Table::drop().table(JwtTokenCode::Table).to_owned())
            .await?;

        Ok(())
    }
}
