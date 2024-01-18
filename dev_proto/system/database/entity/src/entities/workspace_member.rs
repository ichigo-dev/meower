//------------------------------------------------------------------------------
//! WorkspaceMember model.
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
    #[error("WorkspaceMember: Database error.")]
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
    pub fn get_column_name( &self ) -> String
    {
        match self
        {
            Self::WorkspaceMemberId => t!("entities.workspace_member.workspace_member_id.name"),
            Self::WorkspaceId => t!("entities.workspace_member.workspace_id.name"),
            Self::UserAccountId => t!("entities.workspace_member.user_account_id.name"),
            Self::WorkspaceMemberAuthorityId => t!("entities.workspace_member.workspace_member_authority_id.name"),
        }
        .to_string()
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace_member")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub workspace_member_id: i64,
    pub workspace_id: i64,
    pub user_account_id: i64,
    pub workspace_member_authority_id: i64,
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
        belongs_to = "super::user_account::Entity",
        from = "Column::UserAccountId",
        to = "super::user_account::Column::UserAccountId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserAccount,

    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::WorkspaceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Workspace,

    #[sea_orm(
        belongs_to = "super::workspace_member_authority::Entity",
        from = "Column::WorkspaceMemberAuthorityId",
        to = "super::workspace_member_authority::Column::WorkspaceMemberAuthorityId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    WorkspaceMemberAuthority,
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}

impl Related<super::workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Workspace.def()
    }
}

impl Related<super::workspace_member_authority::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::WorkspaceMemberAuthority.def()
    }
}
