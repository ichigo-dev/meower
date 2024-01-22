//------------------------------------------------------------------------------
//! Creates reset_password_token table.
//------------------------------------------------------------------------------

use crate::table_def::{ ResetPasswordToken, User };

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
            .table(ResetPasswordToken::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(ResetPasswordToken::ResetPasswordTokenId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(ResetPasswordToken::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ResetPasswordToken::Token)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(ResetPasswordToken::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ResetPasswordToken::ExpiredAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("reset_password_token_user_id_fkey")
                    .from(ResetPasswordToken::Table, ResetPasswordToken::UserId)
                    .to(User::Table, User::UserId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("reset_password_token_user_id_idx")
            .table(ResetPasswordToken::Table)
            .col(ResetPasswordToken::UserId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"reset_password_token\" IS 'Reset password table';",
            "COMMENT ON COLUMN \"reset_password_token\".\"reset_password_token_id\" IS 'Reset password token ID';",
            "COMMENT ON COLUMN \"reset_password_token\".\"user_id\" IS 'User ID';",
            "COMMENT ON COLUMN \"reset_password_token\".\"token\" IS 'Token';",
            "COMMENT ON COLUMN \"reset_password_token\".\"created_at\" IS 'Create date';",
            "COMMENT ON COLUMN \"reset_password_token\".\"expired_at\" IS 'Expire date';",
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
            .drop_table(Table::drop().table(ResetPasswordToken::Table).to_owned())
            .await?;

        Ok(())
    }
}
