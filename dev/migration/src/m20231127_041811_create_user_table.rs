//------------------------------------------------------------------------------
//! Creates `user` table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;


//------------------------------------------------------------------------------
/// User table.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
enum User
{
    Table,
    UserId,
    Email,
    Password,
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
    /// Creates `user` table.
    //--------------------------------------------------------------------------
    async fn up( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .create_table
            (
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col
                    (
                        ColumnDef::new(User::UserId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col
                    (
                        ColumnDef::new(User::Email)
                            .string()
                            .unique_key()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(User::CreatedAt)
                            .date_time()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(User::UpdatedAt)
                            .date_time()
                            .not_null()
                    )
                    .col
                    (
                        ColumnDef::new(User::IsDeleted)
                            .boolean()
                            .default(false)
                    )
                    .to_owned()
            )
            .await
    }

    //--------------------------------------------------------------------------
    /// Drops `user` table.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
