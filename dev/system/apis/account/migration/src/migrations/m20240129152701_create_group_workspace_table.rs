//------------------------------------------------------------------------------
//! Creates group_workspace table.
//------------------------------------------------------------------------------

use crate::table_def::{ Group, GroupWorkspace, Workspace };

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
            .table(GroupWorkspace::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(GroupWorkspace::GroupWorkspaceId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(GroupWorkspace::WorkspaceId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupWorkspace::GroupId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_workspace_workspace_id_fk")
                    .from(GroupWorkspace::Table, GroupWorkspace::WorkspaceId)
                    .to(Workspace::Table, Workspace::WorkspaceId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_workspace_group_id_fk")
                    .from(GroupWorkspace::Table, GroupWorkspace::GroupId)
                    .to(Group::Table, Group::GroupId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("group_workspace_workspace_id_idx")
            .table(GroupWorkspace::Table)
            .col(GroupWorkspace::WorkspaceId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("group_workspace_group_id_idx")
            .table(GroupWorkspace::Table)
            .col(GroupWorkspace::GroupId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group_workspace\" IS 'Group workspace table'",
            "COMMENT ON COLUMN \"group_workspace\".\"group_workspace_id\" IS 'Group workspace ID'",
            "COMMENT ON COLUMN \"group_workspace\".\"workspace_id\" IS 'Workspace ID'",
            "COMMENT ON COLUMN \"group_workspace\".\"group_id\" IS 'Group ID'",
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
            .drop_table(Table::drop().table(GroupWorkspace::Table).to_owned())
            .await?;

        Ok(())
    }
}
