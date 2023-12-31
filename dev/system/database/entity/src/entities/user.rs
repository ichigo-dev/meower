//------------------------------------------------------------------------------
//! User model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;
use super::user_auth::Entity as UserAuthEntity;
use super::user_account::{ self, Entity as UserAccountEntity };
use super::user_account::Model as UserAccountModel;

use meower_validator::{ Validator, ValidationError };

use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("User: The email already exists.")]
    EmailAlreadyExists,

    #[error("User: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("User: Database error.")]
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
            Self::EmailAlreadyExists =>
            {
                return
                (
                    Some(Column::Email),
                    t!("entities.user.email.error.already_exists"),
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
            Self::UserId => t!("entities.user.user_id.name"),
            Self::Email => t!("entities.user.email.name"),
            Self::CreatedAt => t!("entities.user.created_at.name"),
            Self::UpdatedAt => t!("entities.user.updated_at.name"),
            Self::IsDeleted => t!("entities.user.is_deleted.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Entity.
//------------------------------------------------------------------------------
impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds user by email.
    //--------------------------------------------------------------------------
    pub fn find_by_email( email: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Email.eq(email))
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_id: i64,
    #[sea_orm(unique)]
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Gets last logined user account.
    //--------------------------------------------------------------------------
    pub async fn get_last_logined_user_account<C>
    (
        &self,
        hdb: &C,
    ) -> Option<UserAccountModel>
    where
        C: ConnectionTrait,
    {
        UserAccountEntity::find()
            .filter(user_account::Column::UserId.eq(self.user_id))
            .order_by_desc(user_account::Column::LastLoginedAt)
            .one(hdb)
            .await
            .unwrap_or(None)
    }

    //--------------------------------------------------------------------------
    /// Tries to login.
    //--------------------------------------------------------------------------
    pub async fn try_login<C>( &self, hdb: &C, password: &str ) -> bool
    where
        C: ConnectionTrait,
    {
        if let Some(user_auth) = self
            .find_related(UserAuthEntity)
            .one(hdb)
            .await
            .unwrap_or(None)
        {
            return user_auth.verify_password(password);
        }
        false
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
            self.set(Column::IsDeleted, false.into());
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
        let email = self.email.clone().take().unwrap_or("".to_string());

        // Checks if the user already exists.
        if self.user_id.is_set() == false
        {
            if Entity::find_by_email(&email)
                .one(hdb)
                .await
                .unwrap_or(None)
                .is_some()
            {
                return Err(Error::EmailAlreadyExists);
            }
        }

        // Validates fields.
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
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::user_account::Entity")]
    UserAccount,

    #[sea_orm(has_one = "super::user_auth::Entity")]
    UserAuth,

    #[sea_orm(has_one = "super::reset_password_token::Entity")]
    ResetPasswordToken,

    #[sea_orm(has_one = "super::user_jwt_subject::Entity")]
    UserJwtSubject,
}

impl Related<super::user_account::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAccount.def()
    }
}

impl Related<super::user_auth::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAuth.def()
    }
}

impl Related<super::reset_password_token::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ResetPasswordToken.def()
    }
}

impl Related<super::user_jwt_subject::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserJwtSubject.def()
    }
}
