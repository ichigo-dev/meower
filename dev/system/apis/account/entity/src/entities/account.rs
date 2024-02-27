//------------------------------------------------------------------------------
//! Account model.
//------------------------------------------------------------------------------

use super::account_profile::Entity as AccountProfileEntity;
use super::account_profile::Model as AccountProfileModel;
use super::account_workspace::Entity as AccountWorkspaceEntity;
use super::account_workspace::Model as AccountWorkspaceModel;
use super::entity_linked::{
    AccountToWorkspace,
    AccountToGroup,
    AccountToGroupWorkspace,
};
use super::workspace::Entity as WorkspaceEntity;
use super::workspace::Model as WorkspaceModel;
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
#[sea_orm(table_name = "account")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub account_id: i64,
    #[sea_orm(unique)]
    pub account_name: String,
    pub email: String,
    pub public_user_id: String,
    pub default_account_profile_id: i64,
    pub default_workspace_id: i64,
    pub created_at: DateTime,
    pub last_login_at: DateTime,
}

#[Object(name = "Account")]
impl Model
{
    //--------------------------------------------------------------------------
    /// Gets the account name.
    //--------------------------------------------------------------------------
    pub async fn account_name( &self ) -> String
    {
        self.account_name.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the email.
    //--------------------------------------------------------------------------
    pub async fn email( &self ) -> String
    {
        self.email.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the public user id.
    //--------------------------------------------------------------------------
    pub async fn public_user_id( &self ) -> String
    {
        self.public_user_id.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the create date.
    //--------------------------------------------------------------------------
    pub async fn created_at( &self ) -> DateTime
    {
        self.created_at
    }

    //--------------------------------------------------------------------------
    /// Gets the last login date.
    //--------------------------------------------------------------------------
    pub async fn last_login_at( &self ) -> DateTime
    {
        self.last_login_at
    }

    //--------------------------------------------------------------------------
    /// Gets the default account profile.
    //--------------------------------------------------------------------------
    pub async fn default_account_profile
    (
        &self,
        ctx: &Context<'_>,
    ) -> Option<AccountProfileModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        AccountProfileEntity::find_by_id(self.default_account_profile_id)
            .one(tsx)
            .await
            .unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the default workspace.
    //--------------------------------------------------------------------------
    pub async fn default_workspace
    (
        &self,
        ctx: &Context<'_>,
    ) -> Option<WorkspaceModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        WorkspaceEntity::find_by_id(self.default_workspace_id)
            .one(tsx)
            .await
            .unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the workspaces. (Account workspaces and member group workspaces)
    //--------------------------------------------------------------------------
    pub async fn workspaces( &self, ctx: &Context<'_> ) -> Vec<WorkspaceModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let account_workspaces = self.find_linked(AccountToWorkspace)
            .all(tsx)
            .await
            .unwrap();
        let group_workspaces = self.find_linked(AccountToGroupWorkspace)
            .all(tsx)
            .await
            .unwrap();
        [account_workspaces, group_workspaces].concat()
    }

    //--------------------------------------------------------------------------
    /// Gets the account profiles.
    //--------------------------------------------------------------------------
    pub async fn account_profiles
    (
        &self,
        ctx: &Context<'_>,
    ) -> Vec<AccountProfileModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(AccountProfileEntity).all(tsx).await.unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the account workspaces.
    //--------------------------------------------------------------------------
    pub async fn account_workspaces
    (
        &self,
        ctx: &Context<'_>,
    ) -> Vec<AccountWorkspaceModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_related(AccountWorkspaceEntity).all(tsx).await.unwrap()
    }

    //--------------------------------------------------------------------------
    /// Gets the member groups.
    //--------------------------------------------------------------------------
    pub async fn member_groups
    (
        &self,
        ctx: &Context<'_>,
    ) -> Vec<super::group::Model>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        self.find_linked(AccountToGroup).all(tsx).await.unwrap()
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
        let email = self.email
            .clone()
            .take()
            .unwrap_or("".to_string());

        // Checks if the user already exists.
        if self.get_primary_key_value().is_none()
        {
            if Entity::find()
                .filter(Column::AccountName.contains(account_name.clone()))
                .one(hdb)
                .await?
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

        if let Err(e) = Validator::new()
            .required()
            .max_length(255)
            .is_email()
            .validate(&email)
        {
            return Err(Error::Validation { column: Column::Email, error: e });
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
            Self::Email => t!("entities.account.email.name"),
            Self::PublicUserId => t!("entities.account.public_user_id.name"),
            Self::DefaultAccountProfileId => t!("entities.account.default_account_profile_id.name"),
            Self::DefaultWorkspaceId => t!("entities.account.default_workspace_id.name"),
            Self::CreatedAt => t!("entities.account.created_at.name"),
            Self::LastLoginAt => t!("entities.account.last_login_at.name"),
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
    #[sea_orm(
        belongs_to = "super::account_profile::Entity",
        from = "Column::DefaultAccountProfileId",
        to = "super::account_profile::Column::AccountProfileId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    DefaultAccountProfile,

    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::DefaultWorkspaceId",
        to = "super::workspace::Column::WorkspaceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    DefaultWorkspace,

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
