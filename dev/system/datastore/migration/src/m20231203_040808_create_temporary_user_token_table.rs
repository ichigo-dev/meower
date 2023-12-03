//------------------------------------------------------------------------------
//! Create temporary_user_token table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::{ TemporaryUser, TemporaryUserToken };


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
            .table(TemporaryUserToken::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(TemporaryUserToken::TemporaryUserTokenId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(TemporaryUserToken::TemporaryUserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(TemporaryUserToken::Token)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(TemporaryUserToken::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("temporary_user_token_temporary_user_id_fkey")
                    .from(TemporaryUserToken::Table, TemporaryUserToken::TemporaryUserId)
                    .to(TemporaryUser::Table, TemporaryUser::TemporaryUserId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("temporary_user_token_temporary_user_id_idx")
            .table(TemporaryUserToken::Table)
            .col(TemporaryUserToken::TemporaryUserId)
            .to_owned();
        manager.create_index(index).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(TemporaryUserToken::Table).to_owned())
            .await
    }
}
