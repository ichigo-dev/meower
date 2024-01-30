//------------------------------------------------------------------------------
//! GroupMember model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use async_graphql::{ Enum, SimpleObject };
use async_trait::async_trait;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Role.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "i8", db_type = "TinyInteger")]
pub enum Role
{
    Admin = 99,
    Member = 30,
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "group_member")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub group_member_id: i64,
    pub group_id: i64,
    pub account_id: i64,
    pub account_profile_id: i64,
    pub role: Role,
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
            Self::GroupMemberId => t!("entities.group_member.group_member_id.name"),
            Self::GroupId => t!("entities.group_member.group_id.name"),
            Self::AccountId => t!("entities.group_member.account_id.name"),
            Self::AccountProfileId => t!("entities.group_member.account_profile_id.name"),
            Self::Role => t!("entities.group_member.role.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("GroupMember: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("GroupMember: Database error.")]
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

    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::AccountId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Account,

    #[sea_orm(
        belongs_to = "super::account_profile::Entity",
        from = "Column::AccountProfileId",
        to = "super::account_profile::Column::AccountProfileId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AccountProfile,
}

impl Related<super::group::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Group.def()
    }
}

impl Related<super::account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Account.def()
    }
}

impl Related<super::account_profile::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::AccountProfile.def()
    }
}
