//------------------------------------------------------------------------------
//! Task model.
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
    #[error("Task: Database error.")]
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
            Self::TaskId => t!("entities.task.task_id.name"),
            Self::ProjectId => t!("entities.task.project_id.name"),
            Self::OwnerUserAccountId => t!("entities.task.owner_user_account_id.name"),
            Self::Title => t!("entities.task.title.name"),
            Self::Content => t!("entities.task.content.name"),
            Self::CreatedAt => t!("entities.task.created_at.name"),
            Self::UpdatedAt => t!("entities.task.updated_at.name"),
            Self::IsDeleted => t!("entities.task.is_deleted.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub task_id: i64,
    pub project_id: i64,
    pub owner_user_account_id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
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
        belongs_to = "super::project::Entity",
        from = "Column::ProjectId",
        to = "super::project::Column::ProjectId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Project,

    #[sea_orm(
        belongs_to = "super::user_account::Entity",
        from = "Column::OwnerUserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,

    #[sea_orm(has_many = "super::task_member::Entity")]
    TaskMember,
}

impl Related<super::project::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Project.def()
    }
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}

impl Related<super::task_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::TaskMember.def()
    }
}
