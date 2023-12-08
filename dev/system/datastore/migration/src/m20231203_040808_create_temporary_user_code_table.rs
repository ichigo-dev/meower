//------------------------------------------------------------------------------
//! Creates temporary_user_code table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use crate::table_def::{ TemporaryUser, TemporaryUserCode };


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
            .table(TemporaryUserCode::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(TemporaryUserCode::TemporaryUserCodeId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(TemporaryUserCode::TemporaryUserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(TemporaryUserCode::Token)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(TemporaryUserCode::Code)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(TemporaryUserCode::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("temporary_user_code_temporary_user_id_fkey")
                    .from(TemporaryUserCode::Table, TemporaryUserCode::TemporaryUserId)
                    .to(TemporaryUser::Table, TemporaryUser::TemporaryUserId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates an index.
        let index = Index::create()
            .name("temporary_user_code_temporary_user_id_idx")
            .table(TemporaryUserCode::Table)
            .col(TemporaryUserCode::TemporaryUserId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"temporary_user_code\" IS 'Temporary user code';",
            "COMMENT ON COLUMN \"temporary_user_code\".\"temporary_user_code_id\" IS 'Temporary user code ID';",
            "COMMENT ON COLUMN \"temporary_user_code\".\"temporary_user_id\" IS 'Temporary user ID';",
            "COMMENT ON COLUMN \"temporary_user_code\".\"code\" IS 'One time code';",
            "COMMENT ON COLUMN \"temporary_user_code\".\"created_at\" IS 'Create date';",
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
            .drop_table(Table::drop().table(TemporaryUserCode::Table).to_owned())
            .await?;

        Ok(())
    }
}
