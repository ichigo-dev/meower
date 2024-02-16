//------------------------------------------------------------------------------
//! User model.
//------------------------------------------------------------------------------

use super::user_auth::Entity as UserAuthEntity;

use meower_entity_ext::ValidateExt;
use meower_validator::{ Validator, ValidationError };

use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


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
    pub public_user_id: String,
    #[sea_orm(unique)]
    pub jwt_subject: String,
    #[sea_orm(unique)]
    pub email: String,
    pub last_login_at: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Tries to login.
    //--------------------------------------------------------------------------
    pub async fn login<C>( &self, hdb: &C, password: &str ) -> bool
    where
        C: ConnectionTrait,
    {
        if let Some(user_auth) = self
            .find_related(UserAuthEntity)
            .one(hdb)
            .await
            .unwrap_or(None)
        {
            if user_auth.verify_password(password)
            {
                let mut user: ActiveModel = self.clone().into();
                user.set(Column::LastLoginAt, Utc::now().naive_utc().into());
                user.save(hdb).await.unwrap();
                return true;
            }
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
            let public_user_id = meower_utility::get_random_str(128);
            let jwt_subject = meower_utility::get_random_str(128);
            self.set(Column::PublicUserId, public_user_id.into());
            self.set(Column::JwtSubject, jwt_subject.into());
            self.set(Column::CreatedAt, now.into());
            self.set(Column::LastLoginAt, now.into());
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
        if self.get_primary_key_value().is_none()
        {
            if Entity::find()
                .filter(Column::Email.contains(email.clone()))
                .one(hdb)
                .await?
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
            Self::PublicUserId => t!("entities.user.public_user_id.name"),
            Self::JwtSubject => t!("entities.user.jwt_subject.name"),
            Self::Email => t!("entities.user.email.name"),
            Self::CreatedAt => t!("entities.user.created_at.name"),
            Self::UpdatedAt => t!("entities.user.updated_at.name"),
            Self::LastLoginAt => t!("entities.user.last_login_at.name"),
            Self::IsDeleted => t!("entities.user.is_deleted.name"),
        }
    }
}


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
    /// Gets the error columns.
    //--------------------------------------------------------------------------
    pub fn get_column( &self ) -> Option<Column>
    {
        match self
        {
            Self::EmailAlreadyExists => Some(Column::Email),
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
            Self::EmailAlreadyExists =>
            {
                t!("entities.user.email.error.already_exists")
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
    #[sea_orm(has_one = "super::user_auth::Entity")]
    UserAuth,
}

impl Related<super::user_auth::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::UserAuth.def()
    }
}
