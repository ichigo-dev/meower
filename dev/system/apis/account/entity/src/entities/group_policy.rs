//------------------------------------------------------------------------------
//! GroupPolicy model.
//------------------------------------------------------------------------------

use super::group::Entity as GroupEntity;
use super::group::Model as GroupModel;
use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

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
#[sea_orm(table_name = "group_policy")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub group_policy_id: i64,
    pub group_id: i64,
    pub raw_policy: String,
    pub updated_at: DateTime,
}

#[Object(name = "GroupPolicy")]
impl Model
{
    //--------------------------------------------------------------------------
    /// Gets the raw policy.
    //--------------------------------------------------------------------------
    pub async fn raw_policy( &self ) -> String
    {
        self.raw_policy.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the group.
    //--------------------------------------------------------------------------
    pub async fn group( &self, ctx: &Context<'_> ) -> Option<GroupModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(GroupEntity).one(tsx).await.unwrap()
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
        _insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Sets the default values.
        let now = Utc::now().naive_utc();
        self.set(Column::UpdatedAt, now.into());
        Ok(self)
    }
}

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
            Self::GroupPolicyId => t!("entities.group_policy.group_policy_id.name"),
            Self::GroupId => t!("entities.group_policy.group_id.name"),
            Self::RawPolicy => t!("entities.group_policy.raw_policy.name"),
            Self::UpdatedAt => t!("entities.group_policy.updated_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("GroupPolicy: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("GroupPolicy: Database error.")]
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
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::GroupId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Group,
}

impl Related<super::group::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Group.def()
    }
}
