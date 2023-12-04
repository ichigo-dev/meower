//------------------------------------------------------------------------------
//! Create task_member table.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;
use crate::table_def::{ UserAccount, Task, TaskMember };


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
            .table(TaskMember::Table)
            .if_not_exists()
            .col
            (
                ColumnDef::new(TaskMember::TaskMemberId)
                    .big_integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col
            (
                ColumnDef::new(TaskMember::TaskId)
                    .big_integer()
                    .not_null()
            )
            .col
            (
                ColumnDef::new(TaskMember::UserAccountId)
                    .big_integer()
                    .not_null()
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("task_member_task_id_fkey")
                    .from(TaskMember::Table, TaskMember::TaskId)
                    .to(Task::Table, Task::TaskId)
            )
            .foreign_key
            (
                ForeignKey::create()
                    .name("task_member_user_account_id_fkey")
                    .from(TaskMember::Table, TaskMember::UserAccountId)
                    .to(UserAccount::Table, UserAccount::UserAccountId)
            )
            .to_owned();
        manager.create_table(table).await?;

        let index = Index::create()
            .name("task_member_task_id_idx")
            .table(TaskMember::Table)
            .col(TaskMember::TaskId)
            .to_owned();
        manager.create_index(index).await?;

        let index = Index::create()
            .name("task_member_user_account_id_idx")
            .table(TaskMember::Table)
            .col(TaskMember::UserAccountId)
            .to_owned();
        manager.create_index(index).await
    }

    //--------------------------------------------------------------------------
    /// Down.
    //--------------------------------------------------------------------------
    async fn down( &self, manager: &SchemaManager ) -> Result<(), DbErr>
    {
        manager
            .drop_table(Table::drop().table(TaskMember::Table).to_owned())
            .await
    }
}
