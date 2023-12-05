//------------------------------------------------------------------------------
//! Creates user_account table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use crate::table_def::{ User, UserAccount };


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
            .table(UserAccount::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserAccount::UserAccountId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserAccount::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccount::UserAccountName)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(UserAccount::DisplayName)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccount::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccount::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccount::IsDeleted)
                    .boolean()
                    .default(false)
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("user_account_user_id_fkey")
                    .from(UserAccount::Table, UserAccount::UserId)
                    .to(User::Table, User::UserId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates an index.
        let index = Index::create()
            .name("user_account_user_id_idx")
            .table(UserAccount::Table)
            .col(UserAccount::UserId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"user_account\" IS 'User account table';",
            "COMMENT ON COLUMN \"user_account\".\"user_account_id\" IS 'User account ID';",
            "COMMENT ON COLUMN \"user_account\".\"user_id\" IS 'User ID';",
            "COMMENT ON COLUMN \"user_account\".\"user_account_name\" IS 'User account name';",
            "COMMENT ON COLUMN \"user_account\".\"display_name\" IS 'Display name';",
            "COMMENT ON COLUMN \"user_account\".\"created_at\" IS 'Create date';",
            "COMMENT ON COLUMN \"user_account\".\"updated_at\" IS 'Update date';",
            "COMMENT ON COLUMN \"user_account\".\"is_deleted\" IS 'Soft delete flag';",
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
            .drop_table(Table::drop().table(UserAccount::Table).to_owned())
            .await?;

        Ok(())
    }
}
