//------------------------------------------------------------------------------
//! Creates user table.
//------------------------------------------------------------------------------

use crate::table_def::User;

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
            .table(User::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(User::UserId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(User::Email)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(User::JwtSubject)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(User::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(User::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(User::LastLoginedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(User::IsDeleted)
                    .boolean()
                    .default(false)
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"user\" IS 'User table'",
            "COMMENT ON COLUMN \"user\".\"user_id\" IS 'User ID'",
            "COMMENT ON COLUMN \"user\".\"email\" IS 'Email address'",
            "COMMENT ON COLUMN \"user\".\"jwt_subject\" IS 'JWT subject'",
            "COMMENT ON COLUMN \"user\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"user\".\"updated_at\" IS 'Update date'",
            "COMMENT ON COLUMN \"user\".\"last_logined_at\" IS 'Last logined date'",
            "COMMENT ON COLUMN \"user\".\"is_deleted\" IS 'Soft delete flag'",
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}
