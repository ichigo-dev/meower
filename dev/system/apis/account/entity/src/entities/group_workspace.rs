//------------------------------------------------------------------------------
//! GroupWorkspace model.
//------------------------------------------------------------------------------

use super::group::Entity as GroupEntity;
use super::group::Model as GroupModel;
use super::workspace::Entity as WorkspaceEntity;
use super::workspace::Model as WorkspaceModel;
use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use std::sync::Arc;

use async_graphql::{ Context, Object };
use async_trait::async_trait;
use rust_i18n::t;
use sea_orm::DatabaseTransaction;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "group_workspace")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub group_workspace_id: i64,
    pub workspace_id: i64,
    pub group_id: i64,
}

#[Object(name = "GroupWorkspace")]
impl Model
{
    //--------------------------------------------------------------------------
    /// Gets the group.
    //--------------------------------------------------------------------------
    pub async fn group( &self, ctx: &Context<'_> ) -> Option<GroupModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(GroupEntity).one(tsx).await.unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the workspace.
    //--------------------------------------------------------------------------
    pub async fn workspace( &self, ctx: &Context<'_> ) -> Option<WorkspaceModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(WorkspaceEntity).one(tsx).await.unwrap()
    }
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[async_trait]
impl ValidateExt for ActiveModel
{
    type Error = Error;
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
            Self::GroupWorkspaceId => t!("entities.group_workspace.group_workspace_id.name"),
            Self::WorkspaceId => t!("entities.group_workspace.workspace_id.name"),
            Self::GroupId => t!("entities.group_workspace.group_id.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("GroupWorkspace: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("GroupWorkspace: Database error.")]
    DbError(#[from] DbErr),
}

impl Error
{
    //--------------------------------------------------------------------------
    /// Gets the error columns.
    //--------------------------------------------------------------------------
    pub fn get_column( &self ) -> Option<Column>
    {
        match self
        {
            Self::Validation { column, .. } => Some(*column),
            Self::DbError(_) => None,
        }
    }

    //--------------------------------------------------------------------------
    /// Gets the error message.
    //--------------------------------------------------------------------------
    pub fn get_message( &self ) -> String
    {
        match self
        {
            Self::Validation { column, error } =>
            {
                error.get_error_message(&column.get_name())
            },
            Self::DbError(_) => t!("common.error.db"),
        }
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::WorkspaceId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Workspace,

    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::GroupId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Group,
}

impl Related<super::workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Workspace.def()
    }
}

impl Related<super::group::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Group.def()
    }
}
