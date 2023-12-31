//------------------------------------------------------------------------------
//! UserAuth model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;
use crate::utils::hash;

use meower_validator::{ Validator, ValidationError };

use argon2::PasswordHash;
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
    #[error("UserAuth: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("UserAuth: Database error.")]
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
            Self::UserAuthId => t!("entities.user_auth.user_auth_id.name"),
            Self::UserId => t!("entities.user_auth.user_id.name"),
            Self::Password => t!("entities.user_auth.password.name"),
            Self::CreatedAt => t!("entities.user_auth.created_at.name"),
            Self::UpdatedAt => t!("entities.user_auth.updated_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_auth")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_auth_id: i64,
    pub user_id: i64,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Verifies the password.
    //--------------------------------------------------------------------------
    pub fn verify_password( &self, password: &str ) -> bool
    {
        hash::verify_field(password, &self.password)
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
        hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Deletes the old datas.
        if insert
        {
            let user_id = self.user_id.clone().take().unwrap_or(0);
            Entity::delete_many()
                .filter(Column::UserId.eq(user_id))
                .exec(hdb)
                .await?;
        }

        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            self.set(Column::CreatedAt, now.into());
        }
        self.set(Column::UpdatedAt, now.into());

        // Hashes the password.
        let password = self.password.clone().take().unwrap_or("".to_string());
        if let Err(_) = PasswordHash::new(&password)
        {
            self.set(Column::Password, hash::hash_field(&password).into());
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
    async fn validate<C>( &self, _hdb: &C ) -> Result<(), Self::Error>
    where
        C: ConnectionTrait,
    {
        let password = self.password.clone().take().unwrap_or("".to_string());

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .min_length(8)
            .max_length(255)
            .regex(r".*[a-zA-Z].*")
            .regex(r".*[0-9].*")
            .regex(r".*[!@#$%^&*()].*")
            .validate(&password)
        {
            return Err(Error::Validation{ column: Column::Password, error: e });
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
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::User.def()
    }
}
