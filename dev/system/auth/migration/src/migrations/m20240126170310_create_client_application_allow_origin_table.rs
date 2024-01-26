//------------------------------------------------------------------------------
//! Creates client_application table.
//------------------------------------------------------------------------------

use crate::table_def::{ ClientApplication, ClientApplicationAllowOrigin };

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
            .table(ClientApplicationAllowOrigin::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(ClientApplicationAllowOrigin::ClientApplicationAllowOriginId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(ClientApplicationAllowOrigin::ClientApplicationId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ClientApplicationAllowOrigin::AllowOrigin)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("client_application_allow_origin_client_application_id_fkey")
                    .from(ClientApplicationAllowOrigin::Table, ClientApplicationAllowOrigin::ClientApplicationId)
                    .to(ClientApplication::Table, ClientApplication::ClientApplicationId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("client_application_allow_origin_client_application_id_idx")
            .table(ClientApplicationAllowOrigin::Table)
            .col(ClientApplicationAllowOrigin::ClientApplicationId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"client_application_allow_origin\" IS 'Client application allow origin table';",
            "COMMENT ON COLUMN \"client_application_allow_origin\".\"client_application_allow_origin_id\" IS 'Client application allow origin ID';",
            "COMMENT ON COLUMN \"client_application_allow_origin\".\"client_application_id\" IS 'Client application id';",
            "COMMENT ON COLUMN \"client_application_allow_origin\".\"allow_origin\" IS 'Allow origin';",
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
            .drop_table(Table::drop().table(ClientApplicationAllowOrigin::Table).to_owned())
            .await?;

        Ok(())
    }
}
