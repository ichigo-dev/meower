//------------------------------------------------------------------------------
//! Creates group_cover table.
//------------------------------------------------------------------------------

use crate::table_def::{ Group, GroupCover };

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
            .table(GroupCover::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(GroupCover::GroupCoverId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(GroupCover::GroupId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupCover::FileKey)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(GroupCover::FileName)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(GroupCover::FileSize)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupCover::ContentType)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupCover::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_cover_group_id_fkey")
                    .from(GroupCover::Table, GroupCover::GroupId)
                    .to(Group::Table, Group::GroupId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("group_cover_group_id_idx")
            .table(GroupCover::Table)
            .col(GroupCover::GroupId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group_cover\" IS 'Group cover table'",
            "COMMENT ON COLUMN \"group_cover\".\"group_id\" IS 'Group ID'",
            "COMMENT ON COLUMN \"group_cover\".\"file_key\" IS 'Cover file key'",
            "COMMENT ON COLUMN \"group_cover\".\"file_name\" IS 'Cover file name'",
            "COMMENT ON COLUMN \"group_cover\".\"file_size\" IS 'Cover file size'",
            "COMMENT ON COLUMN \"group_cover\".\"content_type\" IS 'Cover file content type'",
            "COMMENT ON COLUMN \"group_cover\".\"created_at\" IS 'Create date'",
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
            .drop_table(Table::drop().table(GroupCover::Table).to_owned())
            .await?;

        Ok(())
    }
}
