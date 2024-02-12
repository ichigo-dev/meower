//------------------------------------------------------------------------------
//! Creates account_profile table.
//------------------------------------------------------------------------------

use crate::table_def::{ Account, AccountProfile };

use sea_orm::Statement;
use sea_orm::entity::prelude::DateTime;
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
            .table(AccountProfile::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(AccountProfile::AccountProfileId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(AccountProfile::AccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfile::Name)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfile::Affiliation)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(AccountProfile::Bio)
                    .text()
            )
            .col
            (
                ColumnDef::new(AccountProfile::Email)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfile::Birthdate)
                    .timestamp()
                    .default(DateTime::from_timestamp_millis(0))
            )
            .col
            (
                ColumnDef::new(AccountProfile::Gender)
                    .integer()
            )
            .col
            (
                ColumnDef::new(AccountProfile::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfile::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("account_profile_account_id_fkey")
                    .from(AccountProfile::Table, AccountProfile::AccountId)
                    .to(Account::Table, Account::AccountId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("account_profile_account_id_idx")
            .table(AccountProfile::Table)
            .col(AccountProfile::AccountId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"account_profile\" IS 'Account profile table'",
            "COMMENT ON COLUMN \"account_profile\".\"account_profile_id\" IS 'Account profile ID'",
            "COMMENT ON COLUMN \"account_profile\".\"account_id\" IS 'Account ID'",
            "COMMENT ON COLUMN \"account_profile\".\"name\" IS 'Name'",
            "COMMENT ON COLUMN \"account_profile\".\"affiliation\" IS 'Affiliation'",
            "COMMENT ON COLUMN \"account_profile\".\"bio\" IS 'Biography'",
            "COMMENT ON COLUMN \"account_profile\".\"email\" IS 'Email'",
            "COMMENT ON COLUMN \"account_profile\".\"birthdate\" IS 'Birthdate'",
            "COMMENT ON COLUMN \"account_profile\".\"gender\" IS 'Gender'",
            "COMMENT ON COLUMN \"account_profile\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"account_profile\".\"updated_at\" IS 'Update date'",
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
            .drop_table(Table::drop().table(AccountProfile::Table).to_owned())
            .await?;

        Ok(())
    }
}
