//------------------------------------------------------------------------------
//! Creates user_auth table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use crate::table_def::{ User, UserAuth };


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
            .table(UserAuth::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserAuth::UserAuthId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserAuth::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAuth::Password)
                    .string()
                    .string_len(255)
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAuth::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserAuth::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("user_auth_user_id_fkey")
                    .from(UserAuth::Table, UserAuth::UserId)
                    .to(User::Table, User::UserId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("user_auth_user_id_idx")
            .table(UserAuth::Table)
            .col(UserAuth::UserId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"user_auth\" IS 'User authentication table';",
            "COMMENT ON COLUMN \"user_auth\".\"user_auth_id\" IS 'User authentication ID';",
            "COMMENT ON COLUMN \"user_auth\".\"user_id\" IS 'User ID';",
            "COMMENT ON COLUMN \"user_auth\".\"password\" IS 'Hashed password';",
            "COMMENT ON COLUMN \"user_auth\".\"created_at\" IS 'Create date';",
            "COMMENT ON COLUMN \"user_auth\".\"updated_at\" IS 'Update date';",
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
            .drop_table(Table::drop().table(UserAuth::Table).to_owned())
            .await?;

        Ok(())
    }
}
