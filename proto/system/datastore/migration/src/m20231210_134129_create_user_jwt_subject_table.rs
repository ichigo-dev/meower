//------------------------------------------------------------------------------
//! Creates user_jwt_subject table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use crate::table_def::{ User, UserJwtSubject };


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
            .table(UserJwtSubject::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(UserJwtSubject::UserJwtSubjectId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(UserJwtSubject::UserId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(UserJwtSubject::Subject)
                    .string()
                    .string_len(255)
                    .not_null()
                    .unique_key()
            )
            .col
            (
                ColumnDef::new(UserJwtSubject::CreatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("user_jwt_subject_user_id_fkey")
                    .from(UserJwtSubject::Table, UserJwtSubject::UserId)
                    .to(User::Table, User::UserId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("user_jwt_subject_user_id_idx")
            .table(UserJwtSubject::Table)
            .col(UserJwtSubject::UserId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"user_jwt_subject\" IS 'User JWT subject'",
            "COMMENT ON COLUMN \"user_jwt_subject\".\"user_jwt_subject_id\" IS 'User JWT subject ID'",
            "COMMENT ON COLUMN \"user_jwt_subject\".\"user_id\" IS 'User ID'",
            "COMMENT ON COLUMN \"user_jwt_subject\".\"subject\" IS 'Subject'",
            "COMMENT ON COLUMN \"user_jwt_subject\".\"created_at\" IS 'Create date'",
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
            .drop_table(Table::drop().table(UserJwtSubject::Table).to_owned())
            .await?;

        Ok(())
    }
}
