//------------------------------------------------------------------------------
//! ProjectMember model.
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
    #[error("ProjectMember: Database error.")]
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
            Self::ProjectMemberId => t!("entities.project_member.project_member_id.name"),
            Self::ProjectId => t!("entities.project_member.project_id.name"),
            Self::UserAccountId => t!("entities.project_member.user_account_id.name"),
            Self::ProjectMemberAuthorityId => t!("entities.project_member.project_member_authority_id.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project_member")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub project_member_id: i64,
    pub project_id: i64,
    pub user_account_id: i64,
    pub project_member_authority_id: i64,
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
        belongs_to = "super::project_member_authority::Entity",
        from = "Column::ProjectMemberAuthorityId",
        to = "super::project_member_authority::Column::ProjectMemberAuthorityId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ProjectMemberAuthority,

    #[sea_orm(
        belongs_to = "super::user_account::Entity",
        from = "Column::UserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,
}

impl Related<super::project::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Project.def()
    }
}

impl Related<super::project_member_authority::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ProjectMemberAuthority.def()
    }
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}
