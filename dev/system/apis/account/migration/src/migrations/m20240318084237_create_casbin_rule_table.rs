//------------------------------------------------------------------------------
//! Creates casbin_rule table.
//------------------------------------------------------------------------------

use crate::table_def::CasbinRule;

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
            .table(CasbinRule::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(CasbinRule::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(CasbinRule::Ptype)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(CasbinRule::V0)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(CasbinRule::V1)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(CasbinRule::V2)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(CasbinRule::V3)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(CasbinRule::V4)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(CasbinRule::V5)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .unique()
            .name("unique_key_sqlx_adapter")
            .table(CasbinRule::Table)
            .col(CasbinRule::Ptype)
            .col(CasbinRule::V0)
            .col(CasbinRule::V1)
            .col(CasbinRule::V2)
            .col(CasbinRule::V3)
            .col(CasbinRule::V4)
            .col(CasbinRule::V5)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"casbin_rule\" IS 'Casbin rule table (for Casbin library)'",
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
            .drop_table(Table::drop().table(CasbinRule::Table).to_owned())
            .await?;

        Ok(())
    }
}
