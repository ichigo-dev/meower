//------------------------------------------------------------------------------
//! Account model.
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
#[sea_orm(table_name = "account")]
#[graphql(concrete(name = "Account", params()))]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub account_id: i64,
    #[sea_orm(unique)]
    pub account_name: String,
    pub public_user_id: String,
    pub created_at: DateTime,
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
        if insert
        {
            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());
        }

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
        let account_name = self
            .account_name
            .clone()
            .take()
            .unwrap_or("".to_string());

        // Checks if the user already exists.
        if self.account_id.is_set() == false
        {
            if Entity::find()
                .filter(Column::AccountName.contains(account_name.clone()))
                .one(hdb)
                .await
                .unwrap_or(None)
                .is_some()
            {
                return Err(Error::AccountNameAlreadyExists);
            }
        }

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .min_length(4)
            .max_length(32)
            .regex(r"^[a-zA-Z0-9_]+$")
            .validate(&account_name)
        {
            return Err
            (
                Error::Validation { column: Column::AccountName, error: e }
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
            Self::AccountId => t!("entities.account.account_id.name"),
            Self::AccountName => t!("entities.account.account_name.name"),
            Self::PublicUserId => t!("entities.account.public_user_id.name"),
            Self::CreatedAt => t!("entities.account.created_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("Account: The account_name already exists.")]
    AccountNameAlreadyExists,

    #[error("Account: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("Account: Database error.")]
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
            Self::AccountNameAlreadyExists => Some(Column::AccountName),
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
            Self::AccountNameAlreadyExists =>
            {
                t!("entities.account.account_name.error.already_exists")
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
    #[sea_orm(has_many = "super::account_profile::Entity")]
    AccountProfile,

    #[sea_orm(has_many = "super::account_workspace::Entity")]
    AccountWorkspace,

    #[sea_orm(has_many = "super::group_member::Entity")]
    GroupMember,
}

impl Related<super::account_profile::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::AccountProfile.def()
    }
}

impl Related<super::account_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::AccountWorkspace.def()
    }
}

impl Related<super::group_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::GroupMember.def()
    }
}
