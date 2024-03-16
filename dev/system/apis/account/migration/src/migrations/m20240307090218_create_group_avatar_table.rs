//------------------------------------------------------------------------------
//! Creates group_avatar table.
//------------------------------------------------------------------------------

use crate::table_def::{ Group, GroupAvatar };

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
            .table(GroupAvatar::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(GroupAvatar::GroupAvatarId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(GroupAvatar::GroupId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupAvatar::FileKey)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(GroupAvatar::FileName)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(GroupAvatar::FileSize)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupAvatar::ContentType)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupAvatar::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_avatar_group_id_fkey")
                    .from(GroupAvatar::Table, GroupAvatar::GroupId)
                    .to(Group::Table, Group::GroupId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("group_avatar_group_id_idx")
            .table(GroupAvatar::Table)
            .col(GroupAvatar::GroupId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group_avatar\" IS 'Group avatar table'",
            "COMMENT ON COLUMN \"group_avatar\".\"group_avatar_id\" IS 'Group avatar ID'",
            "COMMENT ON COLUMN \"group_avatar\".\"group_id\" IS 'Group ID'",
            "COMMENT ON COLUMN \"group_avatar\".\"file_key\" IS 'Avatar file key'",
            "COMMENT ON COLUMN \"group_avatar\".\"file_name\" IS 'Avatar file name'",
            "COMMENT ON COLUMN \"group_avatar\".\"file_size\" IS 'Avatar file size'",
            "COMMENT ON COLUMN \"group_avatar\".\"content_type\" IS 'Avatar file content type'",
            "COMMENT ON COLUMN \"group_avatar\".\"created_at\" IS 'Create date'",
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
            .drop_table(Table::drop().table(GroupAvatar::Table).to_owned())
            .await?;

        Ok(())
    }
}
