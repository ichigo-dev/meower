//------------------------------------------------------------------------------
//! TaskMember model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;

use sea_orm::entity::prelude::*;
use rust_i18n::t;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("TaskMember: Database error.")]
    DbError(#[from] DbErr),
}

impl Error
{
    //--------------------------------------------------------------------------
    /// Gets the error message.
    //--------------------------------------------------------------------------
    pub fn get_error_message( &self ) -> (Option<Column>, String)
    {
        match self
        {
            Self::DbError(_) => (None, t!("common.error.db")),
        }
    }
}


//------------------------------------------------------------------------------
/// Column.
//------------------------------------------------------------------------------
impl Column
{
    //--------------------------------------------------------------------------
    /// Gets the column name.
    //--------------------------------------------------------------------------
    pub fn get_name( &self ) -> String
    {
        match self
        {
            Self::TaskMemberId => t!("entities.task_member.task_member_id.name"),
            Self::TaskId => t!("entities.task_member.task_id.name"),
            Self::UserAccountId => t!("entities.task_member.user_account_id.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task_member")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub task_member_id: i64,
    pub task_id: i64,
    pub user_account_id: i64,
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
impl ActiveModelBehavior for ActiveModel {}
impl ValidateExt for ActiveModel { type Error = Error; }


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::task::Entity",
        from = "Column::TaskId",
        to = "super::task::Column::TaskId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Task,

    #[sea_orm(
        belongs_to = "super::user_account::Entity",
        from = "Column::UserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,
}

impl Related<super::task::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Task.def()
    }
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}
