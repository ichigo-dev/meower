//------------------------------------------------------------------------------
//! Creates project_member table.
//------------------------------------------------------------------------------

use crate::table_def::{
    UserAccount,
    Project,
    ProjectMember,
    ProjectMemberAuthority,
};

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
            .table(ProjectMember::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(ProjectMember::ProjectMemberId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(ProjectMember::ProjectId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ProjectMember::UserAccountId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(ProjectMember::ProjectMemberAuthorityId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("project_member_project_id_fkey")
                    .from(ProjectMember::Table, ProjectMember::ProjectId)
                    .to(Project::Table, Project::ProjectId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("project_member_user_account_id_fkey")
                    .from(ProjectMember::Table, ProjectMember::UserAccountId)
                    .to(UserAccount::Table, UserAccount::UserAccountId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("project_member_project_member_authority_fkey")
                    .from(ProjectMember::Table, ProjectMember::ProjectMemberAuthorityId)
                    .to(ProjectMemberAuthority::Table, ProjectMemberAuthority::ProjectMemberAuthorityId)
            )
            .to_owned();
        manager.create_table(table).await?;

        // Creates indexes.
        let index = Index::create()
            .name("project_member_project_id_idx")
            .table(ProjectMember::Table)
            .col(ProjectMember::ProjectId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("project_member_user_account_id_idx")
            .table(ProjectMember::Table)
            .col(ProjectMember::UserAccountId)
            .to_owned();
        manager.create_index(index).await?;

        // Adds comments.
        let querys = vec!
        [
            "COMMENT ON TABLE \"project_member\" IS 'Project member table'",
            "COMMENT ON COLUMN \"project_member\".\"project_member_id\" IS 'Project member ID'",
            "COMMENT ON COLUMN \"project_member\".\"project_id\" IS 'Project ID'",
            "COMMENT ON COLUMN \"project_member\".\"user_account_id\" IS 'User account ID'",
            "COMMENT ON COLUMN \"project_member\".\"project_member_authority_id\" IS 'Project member authority ID'",
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
            .drop_table(Table::drop().table(ProjectMember::Table).to_owned())
            .await?;

        Ok(())
    }
}
