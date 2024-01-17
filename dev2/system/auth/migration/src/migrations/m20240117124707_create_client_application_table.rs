//------------------------------------------------------------------------------
//! Creates client_application table.
//------------------------------------------------------------------------------

use crate::table_def::ClientApplication;

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
            .table(ClientApplication::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(ClientApplication::ClientApplicationId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(ClientApplication::Name)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ClientApplication::ClientId)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(ClientApplication::ClientSecret)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(ClientApplication::RedirectUri)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ClientApplication::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ClientApplication::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"client_application\" IS 'Client application table';",
            "COMMENT ON COLUMN \"client_application\".\"client_application_id\" IS 'Client application ID';",
            "COMMENT ON COLUMN \"client_application\".\"name\" IS 'Client application name';",
            "COMMENT ON COLUMN \"client_application\".\"client_id\" IS 'Client ID';",
            "COMMENT ON COLUMN \"client_application\".\"client_secret\" IS 'Client secret';",
            "COMMENT ON COLUMN \"client_application\".\"redirect_uri\" IS 'Redirect URI';",
            "COMMENT ON COLUMN \"client_application\".\"created_at\" IS 'Create date';",
            "COMMENT ON COLUMN \"client_application\".\"updated_at\" IS 'Update date';",
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
            .drop_table(Table::drop().table(ClientApplication::Table).to_owned())
            .await?;

        Ok(())
    }
}
