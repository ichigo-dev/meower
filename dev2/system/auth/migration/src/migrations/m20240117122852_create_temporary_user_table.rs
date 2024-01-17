//------------------------------------------------------------------------------
//! Creates temporary_user table.
//------------------------------------------------------------------------------

use crate::table_def::TemporaryUser;

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
            .table(TemporaryUser::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(TemporaryUser::TemporaryUserId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(TemporaryUser::Token)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(TemporaryUser::Email)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(TemporaryUser::Password)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(TemporaryUser::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .to_owned();
        manager.create_table(table).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"temporary_user\" IS 'Temporary user table';",
            "COMMENT ON COLUMN \"temporary_user\".\"temporary_user_id\" IS 'Temporary user ID';",
            "COMMENT ON COLUMN \"temporary_user\".\"token\" IS 'Token';",
            "COMMENT ON COLUMN \"temporary_user\".\"email\" IS 'Email';",
            "COMMENT ON COLUMN \"temporary_user\".\"password\" IS 'Password';",
            "COMMENT ON COLUMN \"temporary_user\".\"created_at\" IS 'Created at';",
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
            .drop_table(Table::drop().table(TemporaryUser::Table).to_owned())
            .await?;

        Ok(())
    }
}
