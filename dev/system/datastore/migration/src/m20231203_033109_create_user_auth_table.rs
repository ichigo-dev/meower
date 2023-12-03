//------------------------------------------------------------------------------
//! Create user_auth table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::{ User, UserAuth };


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
            .table(UserAuth::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserAuth::UserAuthId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserAuth::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAuth::Password)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAuth::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAuth::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("user_auth_user_id_fkey")
                    .from(UserAuth::Table, UserAuth::UserId)
                    .to(User::Table, User::UserId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("user_auth_user_id_idx")
            .table(UserAuth::Table)
            .col(UserAuth::UserId)
            .to_owned();
        manager.create_index(index).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(UserAuth::Table).to_owned())
            .await
    }
}
