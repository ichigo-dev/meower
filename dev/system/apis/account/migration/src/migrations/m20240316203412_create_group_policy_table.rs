//------------------------------------------------------------------------------
//! Creates group_policy table.
//------------------------------------------------------------------------------

use crate::table_def::{ Group, GroupPolicy };

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
            .table(GroupPolicy::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(GroupPolicy::GroupPolicyId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(GroupPolicy::GroupId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupPolicy::RawPolicy)
                    .text()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(GroupPolicy::UpdatedAt)
                    .timestamp()
                    .default(Expr::current_timestamp())
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("group_policy_group_id_fkey")
                    .from(GroupPolicy::Table, GroupPolicy::GroupId)
                    .to(Group::Table, Group::GroupId)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("group_policy_group_id_idx")
            .table(GroupPolicy::Table)
            .col(GroupPolicy::GroupId)
            .to_owned();
        manager.create_index(index).await?;

        let querys = vec!
        [
            "COMMENT ON TABLE \"group_policy\" IS 'Group policy table'",
            "COMMENT ON COLUMN \"group_policy\".\"group_policy_id\" IS 'Group policy ID'",
            "COMMENT ON COLUMN \"group_policy\".\"group_id\" IS 'Group ID'",
            "COMMENT ON COLUMN \"group_policy\".\"raw_policy\" IS 'Raw policy'",
            "COMMENT ON COLUMN \"group_policy\".\"updated_at\" IS 'Update date'",
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
            .drop_table(Table::drop().table(GroupPolicy::Table).to_owned())
            .await?;

        Ok(())
    }
}
