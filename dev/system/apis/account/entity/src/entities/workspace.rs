//------------------------------------------------------------------------------
//! Workspace model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::{ Validator, ValidationError };

use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "workspace")]
#[graphql(concrete(name = "Workspace", params()))]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub workspace_id: i64,
    #[sea_orm(unique)]
    pub workspace_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
#[async_trait]
impl ActiveModelBehavior for ActiveModel
{
    //--------------------------------------------------------------------------
    /// Before save.
    //--------------------------------------------------------------------------
    async fn before_save<C>
    (
        mut self,
        _hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());
        }
        self.set(Column::UpdatedAt, now.into());

        Ok(self)
    }
}

#[async_trait]
impl ValidateExt for ActiveModel
{
    type Error = Error;

    //--------------------------------------------------------------------------
    /// Validates the data.
    //--------------------------------------------------------------------------
    async fn validate<C>( &self, hdb: &C ) -> Result<(), Self::Error>
    where
        C: ConnectionTrait,
    {
        let workspace_name = self
            .workspace_name
            .clone()
            .take()
            .unwrap_or("".to_string());

        // Checks if the user already exists.
        if self.workspace_id.is_set() == false
        {
            if Entity::find()
                .filter(Column::WorkspaceName.contains(workspace_name.clone()))
                .one(hdb)
                .await
                .unwrap_or(None)
                .is_some()
            {
                return Err(Error::WorkspaceNameAlreadyExists);
            }
        }

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .min_length(4)
            .max_length(32)
            .regex(r"^[a-zA-Z0-9_]+$")
            .validate(&workspace_name)
        {
            return Err
            (
                Error::Validation { column: Column::WorkspaceName, error: e }
            );
        }

        Ok(())
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
            Self::WorkspaceId => t!("entities.workspace.workspace_id.name"),
            Self::WorkspaceName => t!("entities.workspace.workspace_name.name"),
            Self::CreatedAt => t!("entities.workspace.created_at.name"),
            Self::UpdatedAt => t!("entities.workspace.updated_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("Workspace: The workspace_name already exists.")]
    WorkspaceNameAlreadyExists,

    #[error("Workspace: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("Workspace: Database error.")]
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
            Self::WorkspaceNameAlreadyExists => Some(Column::WorkspaceName),
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
            Self::WorkspaceNameAlreadyExists =>
            {
                t!("entities.workspace.workspace_name.error.already_exists")
            },
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
    #[sea_orm(has_one = "super::account_workspace::Entity")]
    AccountWorkspace,

    #[sea_orm(has_one = "super::group_workspace::Entity")]
    GroupWorkspace,
}

impl Related<super::account_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::AccountWorkspace.def()
    }
}

impl Related<super::group_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::GroupWorkspace.def()
    }
}
