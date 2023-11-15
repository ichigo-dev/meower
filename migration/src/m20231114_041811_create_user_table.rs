use sea_orm_migration::prelude::*;

//------------------------------------------------------------------------------
/// User table.
//------------------------------------------------------------------------------
#[derive(DeriveIden)]
enum User
{
    Table,
    Id,
    Email,
    AccountName,
    Password,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    //--------------------------------------------------------------------------
    /// Creates `user` table.
    //--------------------------------------------------------------------------
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .create_table
            (
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col
                    (
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::AccountName).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .to_owned(),
            )
            .await
    }

    //--------------------------------------------------------------------------
    /// Drops `user` table.
    //--------------------------------------------------------------------------
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
