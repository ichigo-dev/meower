//------------------------------------------------------------------------------
//! Creates `accont` table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
enum User
{
    Table,
    UserId,
}

//------------------------------------------------------------------------------
/// Account table.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
enum Account
{
    Table,
    AccountId,
    UserId,
    AccountName,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    //--------------------------------------------------------------------------
    /// Creates `account` table.
    //--------------------------------------------------------------------------
    async fn up( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .create_table
            (
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col
                    (
                        ColumnDef::new(Account::AccountId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col
                    (
                        ColumnDef::new(Account::UserId)
                            .integer()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(Account::AccountName)
                            .string()
                            .unique_key()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(Account::CreatedAt)
                            .date_time()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(Account::UpdatedAt)
                            .date_time()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(Account::IsDeleted)
                            .boolean()
                            .default(false)
                    )
                    .foreign_key
                    (
                        ForeignKey::create()
                            .name("fk_account_user_id")
                            .from(Account::Table, Account::UserId)
                            .to(User::Table, User::UserId)
                    )
                    .to_owned()
            )
            .await
    }

    //--------------------------------------------------------------------------
    /// Drops `account` table.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}
