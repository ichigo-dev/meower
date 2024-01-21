//------------------------------------------------------------------------------
//! TemporaryUser model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use super::user::ActiveModel as UserActiveModel;
use super::user::Error as UserError;
use super::user::Model as UserModel;
use super::user_auth::ActiveModel as UserAuthActiveModel;
use super::user_auth::Error as UserAuthError;
use super::temporary_user_code::Entity as TemporaryUserCodeEntity;
use super::temporary_user_code::Error as TemporaryUserCodeError;

use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use thiserror::Error;


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
    pub token: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Checks if the code is valid.
    //--------------------------------------------------------------------------
    pub async fn verify_code<C>
    (
        &self,
        hdb: &C,
        code: &str
    ) -> Result<(), Error>
    where
        C: ConnectionTrait,
    {
        let temporary_user_code = match self
            .find_related(TemporaryUserCodeEntity)
            .one(hdb)
            .await?
        {
            Some(temporary_user_code) => temporary_user_code,
            None => return Err(Error::TemporaryUserCodeNotFound),
        };
        temporary_user_code.verify(code)?;
        Ok(())
    }

    //--------------------------------------------------------------------------
    /// Registers a new user.
    //--------------------------------------------------------------------------
    pub async fn register<C>( &self, hdb: &C ) -> Result<UserModel, Error>
    where
        C: ConnectionTrait,
    {
        // Creates a new user.
        let user = UserActiveModel
        {
            email: ActiveValue::Set(self.email.clone()),
            ..Default::default()
        };
        let user = user.validate_and_insert(hdb).await?;

        // Creates a new user_auth.
        let user_auth = UserAuthActiveModel
        {
            user_id: ActiveValue::Set(user.user_id),
            password: ActiveValue::Set(self.password.clone()),
            ..Default::default()
        };
        user_auth.validate_and_insert(hdb).await?;

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
            let token = meower_utility::get_random_str(64);
            self.set(Column::Token, token.into());

            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());
        }

        // Hashes the password.
        let password = self.password.clone().take().unwrap_or("".to_string());
        if meower_utility::is_hashed(&password) == false
        {
            let password = meower_utility::hash_value(&password);
            self.set(Column::Password, password.into());
        };

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
        if Entity::find()
            .filter(Column::Email.eq(&email))
            .one(hdb)
            .await?
            .is_some()
        {
            return Err(Error::EmailAlreadyExists);
        }

        // User validation.
        let user = UserActiveModel
        {
            email: ActiveValue::Set(email),
            ..Default::default()
        };
        user.validate(hdb).await?;

        // UserAuth validation.
        let user_auth = UserAuthActiveModel
        {
            password: ActiveValue::Set(password),
            ..Default::default()
        };
        user_auth.validate(hdb).await?;

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
            Self::TemporaryUserId => t!("entities.temporary_user.temporary_user_id.name"),
            Self::Token => t!("entities.temporary_user.token.name"),
            Self::Email => t!("entities.temporary_user.email.name"),
            Self::Password => t!("entities.temporary_user.password.name"),
            Self::CreatedAt => t!("entities.temporary_user.created_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("TemporaryUser: The email already exists.")]
    EmailAlreadyExists,

    #[error("TemporaryUser: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("TemporaryUser: The temporary user code is not found.")]
    TemporaryUserCodeNotFound,

    #[error("TemporaryUser: {0}")]
    UserError(#[from] UserError),

    #[error("TemporaryUser: {0}")]
    UserAuthError(#[from] UserAuthError),

    #[error("TemporaryUser: {0}")]
    TemporaryUserCodeError(#[from] TemporaryUserCodeError),

    #[error("TemporaryUser: Database error.")]
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
            Self::EmailAlreadyExists => Some(Column::Email),
            Self::Validation { column, .. } => Some(*column),
            Self::TemporaryUserCodeNotFound => None,
            Self::UserError(_) => Some(Column::Email),
            Self::UserAuthError(_) => Some(Column::Password),
            Self::TemporaryUserCodeError(_) => None,
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
            Self::EmailAlreadyExists =>
            {
                t!("entities.temporary_user.email.error.already_exists")
            },
            Self::Validation { column, error } =>
            {
                error.get_error_message(&column.get_name())
            },
            Self::TemporaryUserCodeNotFound =>
            {
                t!("entities.temporary_user.error.code_not_found")
            },
            Self::UserError(error) => error.get_message(),
            Self::UserAuthError(error) => error.get_message(),
            Self::TemporaryUserCodeError(error) => error.get_message(),
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
