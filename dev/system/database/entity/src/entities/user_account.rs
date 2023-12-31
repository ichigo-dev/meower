//------------------------------------------------------------------------------
//! UserAccount model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;

use meower_validator::{ Validator, ValidationError };

use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("UserAccount: The user account name already exists.")]
    UserAccountNameAlreadyExists,

    #[error("UserAccount: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("UserAccount: Database error.")]
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
            Self::UserAccountNameAlreadyExists =>
            {
                return
                (
                    Some(Column::UserAccountName),
                    t!("entities.user_account.user_account_name.error.already_exists"),
                );
            },
            Self::Validation { column, error } =>
            {
                return
                (
                    Some(*column),
                    error.get_error_message(&column.get_name())
                );
            },
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
            Self::UserAccountId => t!("entities.user_account.user_account_id.name"),
            Self::UserId => t!("entities.user_account.user_id.name"),
            Self::UserAccountName => t!("entities.user_account.user_account_name.name"),
            Self::DisplayName => t!("entities.user_account.display_name.name"),
            Self::CreatedAt => t!("entities.user_account.created_at.name"),
            Self::UpdatedAt => t!("entities.user_account.updated_at.name"),
            Self::LastLoginedAt => t!("entities.user_account.last_logined_at.name"),
            Self::IsDeleted => t!("entities.user_account.is_deleted.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Entity.
//------------------------------------------------------------------------------
impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds user_account by user account name.
    //--------------------------------------------------------------------------
    pub fn find_by_user_account_name( user_account_name: &str ) -> Select<Self>
    {
        Self::find().filter(Column::UserAccountName.eq(user_account_name))
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_account")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_account_id: i64,
    pub user_id: i64,
    #[sea_orm(unique)]
    pub user_account_name: String,
    pub display_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub last_logined_at: DateTime,
    pub is_deleted: bool,
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
            self.set(Column::LastLoginedAt, now.into());
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
        let user_account_name = self
            .user_account_name
            .clone()
            .take()
            .unwrap_or("".to_string());

        // Checks if the account already exists.
        if self.user_account_id.is_set() == false
        {
            if Entity::find_by_user_account_name(&user_account_name)
                .one(hdb)
                .await?
                .is_some()
            {
                return Err(Error::UserAccountNameAlreadyExists);
            }
        }

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .min_length(3)
            .max_length(32)
            .regex(r"^[a-zA-Z0-9_]+$")
            .validate(&user_account_name)
        {
            return Err(Error::Validation
            {
                column: Column::UserAccountName,
                error: e,
            });
        };

        let display_name = self
            .display_name
            .clone()
            .take()
            .unwrap_or("".to_string());
        if let Err(e) = Validator::new()
            .required()
            .max_length(32)
            .validate(&display_name)
        {
            return Err(Error::Validation
            {
                column: Column::DisplayName,
                error: e,
            });
        };

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::organization_member::Entity")]
    OrganizationMember,

    #[sea_orm(has_many = "super::project_member::Entity")]
    ProjectMember,

    #[sea_orm(has_many = "super::task::Entity")]
    Task,

    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,

    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMember,

    #[sea_orm(has_one = "super::user_account_profile::Entity")]
    UserAccountProfile,

    #[sea_orm(has_many = "super::user_account_workspace::Entity")]
    UserAccountWorkspace,
}

impl Related<super::organization_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::OrganizationMember.def()
    }
}

impl Related<super::project_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ProjectMember.def()
    }
}

impl Related<super::task::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Task.def()
    }
}

impl Related<super::user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::User.def()
    }
}

impl Related<super::workspace_member::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::WorkspaceMember.def()
    }
}

impl Related<super::user_account_profile::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccountProfile.def()
    }
}

impl Related<super::user_account_workspace::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccountWorkspace.def()
    }
}
