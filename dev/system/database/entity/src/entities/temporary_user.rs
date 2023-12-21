//------------------------------------------------------------------------------
//! TemporaryUser model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;
use crate::utils::{ hash, token };
use super::user::Model as UserModel;
use super::user::ActiveModel as ActiveUser;
use super::user::Error as UserError;
use super::user_auth::ActiveModel as ActiveUserAuth;
use super::user_auth::Error as UserAuthError;
use super::temporary_user_code::Entity as TemporaryUserCodeEntity;
use super::temporary_user_code::Column as TemporaryUserCodeColumn;

use argon2::PasswordHash;
use async_trait::async_trait;
use chrono::Utc;
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
    AlreadyExistsEmail,

    #[error("TemporaryUser: {0}")]
    UserError(#[from] UserError),

    #[error("TemporaryUser: {0}")]
    UserAuthError(#[from] UserAuthError),

    #[error("TemporaryUser: Database error.")]
    DbError(#[from] DbErr),
}


//------------------------------------------------------------------------------
/// Entity.
//------------------------------------------------------------------------------
impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds temporary_user by email.
    //--------------------------------------------------------------------------
    pub fn find_by_email(  email: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Email.eq(email))
    }

    //--------------------------------------------------------------------------
    /// Finds temporary_user by token.
    //--------------------------------------------------------------------------
    pub fn find_by_token( token: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Token.eq(token))
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
    pub token: String,
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

            let token = token::generate_token(None);
            self.set(Column::Token, token.into());
        }

        // Hashes the password.
        let password = self.password.clone().unwrap();
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
        let temporary_user_id = self.temporary_user_id.clone().unwrap();
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
        let email = self.email.clone().unwrap();
        let password = self.password.clone().unwrap();

        // Checks if the email already exists.
        if Entity::find_by_email(&email).one(hdb).await.unwrap().is_some()
        {
            return Err(Error::AlreadyExistsEmail);
        }

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
