//------------------------------------------------------------------------------
//! Creates organization table.
//------------------------------------------------------------------------------

use crate::table_def::Organization;

use sea_orm_migration::prelude::*;
use sea_orm::Statement;


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
            .table(Organization::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(Organization::OrganizationId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(Organization::OrganizationName)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(Organization::DisplayName)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Organization::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Organization::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(Organization::IsDeleted)
                    .boolean()
                    .default(false)
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"organization\" IS 'Organization table'",
            "COMMENT ON COLUMN \"organization\".\"organization_id\" IS 'Organization ID'",
            "COMMENT ON COLUMN \"organization\".\"organization_name\" IS 'Organization name'",
            "COMMENT ON COLUMN \"organization\".\"display_name\" IS 'Display name'",
            "COMMENT ON COLUMN \"organization\".\"created_at\" IS 'Create date'",
            "COMMENT ON COLUMN \"organization\".\"updated_at\" IS 'Update date'",
            "COMMENT ON COLUMN \"organization\".\"is_deleted\" IS 'Soft delete flag'",
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
            .drop_table(Table::drop().table(Organization::Table).to_owned())
            .await?;

        Ok(())
    }
}
