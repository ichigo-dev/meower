//------------------------------------------------------------------------------
//! Group model.
//------------------------------------------------------------------------------

use super::group_member::Entity as GroupMemberEntity;
use super::group_member::Model as GroupMemberModel;
use super::group_workspace::Entity as GroupWorkspaceEntity;
use super::group_workspace::Model as GroupWorkspaceModel;
use meower_entity_ext::ValidateExt;
use meower_validator::{ Validator, ValidationError };

use std::sync::Arc;

use async_graphql::{ Context, Object };
use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::DatabaseTransaction;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "group")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub group_id: i64,
    #[sea_orm(unique)]
    pub group_name: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[Object(name = "Group")]
impl Model
{
    //--------------------------------------------------------------------------
    /// Gets the group name.
    //--------------------------------------------------------------------------
    pub async fn group_name( &self ) -> String
    {
        self.group_name.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the name.
    //--------------------------------------------------------------------------
    pub async fn name( &self ) -> String
    {
        self.name.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the create date.
    //--------------------------------------------------------------------------
    pub async fn created_at( &self ) -> DateTime
    {
        self.created_at
    }

    //--------------------------------------------------------------------------
    /// Gets the update date.
    //--------------------------------------------------------------------------
    pub async fn updated_at( &self ) -> DateTime
    {
        self.updated_at
    }

    //--------------------------------------------------------------------------
    /// Gets the group members.
    //--------------------------------------------------------------------------
    pub async fn group_members
    (
        &self,
        ctx: &Context<'_>,
    ) -> Vec<GroupMemberModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(GroupMemberEntity)
            .all(tsx)
            .await
            .unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the group workspaces.
    //--------------------------------------------------------------------------
    pub async fn group_workspaces
    (
        &self,
        ctx: &Context<'_>,
    ) -> Vec<GroupWorkspaceModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(GroupWorkspaceEntity).all(tsx).await.unwrap()
    }
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
        let group_name = self
            .group_name
            .clone()
            .take()
            .unwrap_or("".to_string());
        let name = self.name.clone().take().unwrap_or("".to_string());

        // Checks if the user already exists.
        if self.get_primary_key_value().is_none()
        {
            if Entity::find()
                .filter(Column::GroupName.contains(group_name.clone()))
                .one(hdb)
                .await?
                .is_some()
            {
                return Err(Error::GroupNameAlreadyExists);
            }
        }

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .min_length(4)
            .max_length(32)
            .regex(r"^[a-zA-Z0-9_]+$")
            .validate(&group_name)
        {
            return Err
            (
                Error::Validation { column: Column::GroupName, error: e }
            );
        }

        if let Err(e) = Validator::new()
            .required()
            .min_length(4)
            .max_length(32)
            .validate(&name)
        {
            return Err(Error::Validation { column: Column::Name, error: e });
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
            Self::GroupId => t!("entities.group.group_id.name"),
            Self::GroupName => t!("entities.group.group_name.name"),
            Self::Name => t!("entities.group.name.name"),
            Self::CreatedAt => t!("entities.group.created_at.name"),
            Self::UpdatedAt => t!("entities.group.updated_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("Group: The group_name already exists.")]
    GroupNameAlreadyExists,

    #[error("Group: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("Group: Database error.")]
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
            Self::GroupNameAlreadyExists => Some(Column::GroupName),
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
            Self::GroupNameAlreadyExists =>
            {
                t!("entities.group.group_name.error.already_exists")
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
    #[sea_orm(has_many = "super::group_member::Entity")]
    GroupMember,

    #[sea_orm(has_many = "super::group_workspace::Entity")]
    GroupWorkspace,
}

impl Related<super::group_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::GroupMember.def()
    }
}

impl Related<super::group_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::GroupWorkspace.def()
    }
}
