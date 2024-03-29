//------------------------------------------------------------------------------
//! Creates group table.
//------------------------------------------------------------------------------

use crate::table_def::Group;

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
            .table(Group::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(Group::GroupId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(Group::GroupName)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(Group::Name)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Group::Description)
                    .text()
            )
            .col
            (
                ColumnDef::new(Group::Representative)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(Group::Location)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(Group::Email)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(Group::Telno)
                    .string()
                    .string_len(255)
            )
            .col
            (
                ColumnDef::new(Group::FoundedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
            )
            .col
            (
                ColumnDef::new(Group::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Group::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Group::IsPublic)
                    .boolean()
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group\" IS 'Group table'",
            "COMMENT ON COLUMN \"group\".\"group_id\" IS 'Group ID'",
            "COMMENT ON COLUMN \"group\".\"group_name\" IS 'Group name'",
            "COMMENT ON COLUMN \"group\".\"name\" IS 'Name'",
            "COMMENT ON COLUMN \"group\".\"description\" IS 'Description'",
            "COMMENT ON COLUMN \"group\".\"representative\" IS 'Representative'",
            "COMMENT ON COLUMN \"group\".\"location\" IS 'Location'",
            "COMMENT ON COLUMN \"group\".\"email\" IS 'Email'",
            "COMMENT ON COLUMN \"group\".\"telno\" IS 'Telno'",
            "COMMENT ON COLUMN \"group\".\"founded_at\" IS 'Founded date'",
            "COMMENT ON COLUMN \"group\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"group\".\"updated_at\" IS 'Update date'",
            "COMMENT ON COLUMN \"group\".\"is_public\" IS 'Is public'",
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
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;

        Ok(())
    }
}
