//------------------------------------------------------------------------------
//! TemporaryUser model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;
use crate::utils::hash;
use super::user::Model as UserModel;
use super::user::ActiveModel as ActiveUser;
use super::user::Error as UserError;
use super::user_auth::ActiveModel as ActiveUserAuth;
use super::user_auth::Error as UserAuthError;
use super::user_account::ActiveModel as ActiveUserAccount;
use super::user_account::Error as UserAccountError;
use super::temporary_user_code::Entity as TemporaryUserCodeEntity;
use super::temporary_user_code::Column as TemporaryUserCodeColumn;

use argon2::PasswordHash;
use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("TemporaryUser: The email already exists.")]
    EmailAlreadyExists,

    #[error("TemporaryUser: The user account name already exists.")]
    UserAccountNameAlreadyExists,

    #[error("TemporaryUser: {0}")]
    UserError(#[from] UserError),

    #[error("TemporaryUser: {0}")]
    UserAuthError(#[from] UserAuthError),

    #[error("TemporaryUser: {0}")]
    UserAccountError(#[from] UserAccountError),

    #[error("TemporaryUser: Database error.")]
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
                    t!("entities.temporary_user.email.error.already_exists"),
                );
            },
            Self::UserAccountNameAlreadyExists =>
            {
                return
                (
                    Some(Column::UserAccountName),
                    t!("entities.temporary_user.user_account_name.error.already_exists"),
                );
            },
            Self::UserError(e) =>
            {
                return (Some(Column::Email), e.get_error_message().1);
            },
            Self::UserAuthError(e) =>
            {
                return (Some(Column::Password), e.get_error_message().1);
            },
            Self::UserAccountError(e) =>
            {
                return (Some(Column::UserAccountName), e.get_error_message().1);
            }
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
            Self::TemporaryUserId => t!("entities.temporary_user.temporary_user_id.name"),
            Self::UserAccountName => t!("entities.temporary_user.user_account_name.name"),
            Self::Email => t!("entities.temporary_user.email.name"),
            Self::Password => t!("entities.temporary_user.password.name"),
            Self::CreatedAt => t!("entities.temporary_user.created_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Entity.
//------------------------------------------------------------------------------
impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds temporary_user by user account name.
    //--------------------------------------------------------------------------
    pub fn find_by_user_account_name( user_account_name: &str ) -> Select<Self>
    {
        Self::find().filter(Column::UserAccountName.eq(user_account_name))
    }

    //--------------------------------------------------------------------------
    /// Finds temporary_user by email.
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
#[sea_orm(table_name = "temporary_user")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub temporary_user_id: i64,
    #[sea_orm(unique)]
    pub user_account_name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Registers a new user.
    //--------------------------------------------------------------------------
    pub async fn register<C>
    (
        &self,
        hdb: &C,
    ) -> Result<UserModel, Box<dyn std::error::Error>>
    where
        C: ConnectionTrait,
    {
        // Creates a new user.
        let user = ActiveUser
        {
            email: ActiveValue::Set(self.email.clone()),
            ..Default::default()
        };
        let user = user.validate_and_insert(hdb).await?;

        // Creates a new user_auth.
        let user_auth = ActiveUserAuth
        {
            user_id: ActiveValue::Set(user.user_id),
            password: ActiveValue::Set(self.password.clone()),
            ..Default::default()
        };
        user_auth.validate_and_insert(hdb).await?;

        // Creates a new user_account
        let user_account = ActiveUserAccount
        {
            user_id: ActiveValue::Set(user.user_id),
            user_account_name: ActiveValue::Set(self.user_account_name.clone()),
            ..Default::default()
        };
        user_account.validate_and_insert(hdb).await?;

        Ok(user)
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

        // Hashes the password.
        let password = self.password.clone().take().unwrap_or("".to_string());
        if let Err(_) = PasswordHash::new(&password)
        {
            self.set(Column::Password, hash::hash_field(&password).into());
        };

        Ok(self)
    }

    //--------------------------------------------------------------------------
    /// Before delete.
    //--------------------------------------------------------------------------
    async fn before_delete<C>( mut self, hdb: &C ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let temporary_user_id = self
            .temporary_user_id
            .clone()
            .take()
            .unwrap_or(0);
        TemporaryUserCodeEntity::delete_many()
            .filter
            (
                TemporaryUserCodeColumn::TemporaryUserId
                    .eq(temporary_user_id)
            )
            .exec(hdb)
            .await?;
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
        let email = self
            .email
            .clone()
            .take()
            .unwrap_or("".to_string());
        let password = self
            .password
            .clone()
            .take()
            .unwrap_or("".to_string());

        // Checks if the email already exists.
        if Entity::find_by_email(&email).one(hdb).await?.is_some()
        {
            return Err(Error::EmailAlreadyExists);
        }
        if Entity::find_by_user_account_name(&user_account_name)
            .one(hdb)
            .await?
            .is_some()
        {
            return Err(Error::UserAccountNameAlreadyExists);
        }

        // UserAccount validation.
        let user_account = ActiveUserAccount
        {
            user_account_name: ActiveValue::Set(user_account_name.clone()),
            display_name: ActiveValue::Set(user_account_name.clone()),
            ..Default::default()
        };
        user_account.validate(hdb).await?;

        // User validation.
        let user = ActiveUser
        {
            email: ActiveValue::Set(email),
            ..Default::default()
        };
        user.validate(hdb).await?;

        // UserAuth validation.
        let user_auth = ActiveUserAuth
        {
            password: ActiveValue::Set(password),
            ..Default::default()
        };
        user_auth.validate(hdb).await?;

        Ok(())
    }
}


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(has_many = "super::temporary_user_code::Entity")]
    TemporaryUserCode,
}

impl Related<super::temporary_user_code::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::TemporaryUserCode.def()
    }
}
