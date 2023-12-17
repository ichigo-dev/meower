//------------------------------------------------------------------------------
//! Creates user_account_profile table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use crate::table_def::{ UserAccount, UserAccountProfile };


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
            .table(UserAccountProfile::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserAccountProfile::UserAccountProfileId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::UserAccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::Name)
                    .string()
                    .string_len(255)
                    .null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::Biography)
                    .text()
                    .null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::Company)
                    .string()
                    .string_len(255)
                    .null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::Title)
                    .string()
                    .string_len(255)
                    .null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::Location)
                    .string()
                    .string_len(255)
                    .null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAccountProfile::IsDeleted)
                    .boolean()
                    .default(false)
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("user_account_profile_user_account_id_fkey")
                    .from(UserAccountProfile::Table, UserAccountProfile::UserAccountId)
                    .to(UserAccount::Table, UserAccount::UserAccountId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("user_account_profile_user_account_id_idx")
            .table(UserAccountProfile::Table)
            .col(UserAccountProfile::UserAccountId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"user_account_profile\" IS 'User account profile table'",
            "COMMENT ON COLUMN \"user_account_profile\".\"user_account_profile_id\" IS 'User account profile ID'",
            "COMMENT ON COLUMN \"user_account_profile\".\"user_account_id\" IS 'User account ID'",
            "COMMENT ON COLUMN \"user_account_profile\".\"name\" IS 'Name'",
            "COMMENT ON COLUMN \"user_account_profile\".\"biography\" IS 'Biography'",
            "COMMENT ON COLUMN \"user_account_profile\".\"company\" IS 'Company'",
            "COMMENT ON COLUMN \"user_account_profile\".\"title\" IS 'Title'",
            "COMMENT ON COLUMN \"user_account_profile\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"user_account_profile\".\"updated_at\" IS 'Update date'",
            "COMMENT ON COLUMN \"user_account_profile\".\"is_deleted\" IS 'Is deleted'",
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
            .drop_table(Table::drop().table(UserAccountProfile::Table).to_owned())
            .await?;

        Ok(())
    }
}
