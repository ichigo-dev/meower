//------------------------------------------------------------------------------
//! OrganizationWorkspace model.
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
    #[error("OrganizationWorkspace: Database error.")]
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
            Self::OrganizationWorkspaceId => t!("entities.organization_workspace.organization_workspace_id.name"),
            Self::OrganizationId => t!("entities.organization_workspace.organization_id.name"),
            Self::WorkspaceId => t!("entities.organization_workspace.workspace_id.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organization_workspace")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub organization_workspace_id: i64,
    pub organization_id: i64,
    pub workspace_id: i64,
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
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizationId",
        to = "super::organization::Column::OrganizationId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organization,

    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::WorkspaceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Workspace,
}

impl Related<super::organization::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Organization.def()
    }
}

impl Related<super::workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Workspace.def()
    }
}
