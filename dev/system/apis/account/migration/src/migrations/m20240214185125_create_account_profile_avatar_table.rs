//------------------------------------------------------------------------------
//! Creates account_profile_avatar table.
//------------------------------------------------------------------------------

use crate::table_def::{ AccountProfile, AccountProfileAvatar };

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
            .table(AccountProfileAvatar::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(AccountProfileAvatar::AccountProfileAvatarId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(AccountProfileAvatar::AccountProfileId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfileAvatar::FileKey)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(AccountProfileAvatar::FileSize)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfileAvatar::ContentType)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(AccountProfileAvatar::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("account_profile_avatar_account_profile_id_fkey")
                    .from(AccountProfileAvatar::Table, AccountProfileAvatar::AccountProfileId)
                    .to(AccountProfile::Table, AccountProfile::AccountProfileId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("account_profile_avatar_account_profile_id_idx")
            .table(AccountProfileAvatar::Table)
            .col(AccountProfileAvatar::AccountProfileId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"account_profile_avatar\" IS 'Account profile avatar table'",
            "COMMENT ON COLUMN \"account_profile_avatar\".\"account_profile_id\" IS 'Account profile ID'",
            "COMMENT ON COLUMN \"account_profile_avatar\".\"file_key\" IS 'Avatar file key'",
            "COMMENT ON COLUMN \"account_profile_avatar\".\"file_size\" IS 'Avatar file size'",
            "COMMENT ON COLUMN \"account_profile_avatar\".\"content_type\" IS 'Avatar file content type'",
            "COMMENT ON COLUMN \"account_profile_avatar\".\"created_at\" IS 'Create date'",
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
            .drop_table(Table::drop().table(AccountProfileAvatar::Table).to_owned())
            .await?;

        Ok(())
    }
}
