//------------------------------------------------------------------------------
//! Creates account_profile_cover table.
//------------------------------------------------------------------------------

use crate::table_def::{ AccountProfile, AccountProfileCover };

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
            .table(AccountProfileCover::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(AccountProfileCover::AccountProfileCoverId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(AccountProfileCover::AccountProfileId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfileCover::FileKey)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(AccountProfileCover::FileName)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(AccountProfileCover::FileSize)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfileCover::ContentType)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfileCover::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("account_profile_cover_account_profile_id_fkey")
                    .from(AccountProfileCover::Table, AccountProfileCover::AccountProfileId)
                    .to(AccountProfile::Table, AccountProfile::AccountProfileId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("account_profile_cover_account_profile_id_idx")
            .table(AccountProfileCover::Table)
            .col(AccountProfileCover::AccountProfileId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"account_profile_cover\" IS 'Account profile cover table'",
            "COMMENT ON COLUMN \"account_profile_cover\".\"account_profile_cover_id\" IS 'Account profile cover ID'",
            "COMMENT ON COLUMN \"account_profile_cover\".\"account_profile_id\" IS 'Account profile ID'",
            "COMMENT ON COLUMN \"account_profile_cover\".\"file_key\" IS 'Cover file key'",
            "COMMENT ON COLUMN \"account_profile_cover\".\"file_name\" IS 'Cover file name'",
            "COMMENT ON COLUMN \"account_profile_cover\".\"file_size\" IS 'Cover file size'",
            "COMMENT ON COLUMN \"account_profile_cover\".\"content_type\" IS 'Cover file content type'",
            "COMMENT ON COLUMN \"account_profile_cover\".\"created_at\" IS 'Create date'",
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
            .drop_table(Table::drop().table(AccountProfileCover::Table).to_owned())
            .await?;

        Ok(())
    }
}
