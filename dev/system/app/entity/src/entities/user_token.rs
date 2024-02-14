//------------------------------------------------------------------------------
//! UserToken model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use async_trait::async_trait;
use chrono::{ Utc, Duration };
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;

const TOKEN_EXPIRATION_HOURS: i64 = 24;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_token")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_token_id: i64,
    #[sea_orm(unique)]
    pub token: String,
    #[sea_orm(unique)]
    pub public_user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: DateTime,
    pub expired_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Verifies token.
    //--------------------------------------------------------------------------
    pub fn verify( &self ) -> bool
    {
        let now = Utc::now().naive_utc();
        self.expired_at > now
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
            let public_user_id = self
                .public_user_id
                .clone()
                .take()
                .unwrap_or("".to_string());
            Entity::delete_many()
                .filter(Column::PublicUserId.eq(public_user_id))
                .exec(hdb)
                .await?;
        }

        // Sets the default values.
        let now = Utc::now().naive_utc();
        if insert
        {
            let token = meower_utility::get_random_str(128);
            self.set(Column::Token, token.into());

            self.set(Column::CreatedAt, now.into());
        }
        let expired_at = now + Duration::hours(TOKEN_EXPIRATION_HOURS);
        self.set(Column::ExpiredAt, expired_at.into());

        Ok(self)
    }
}

#[async_trait]
impl ValidateExt for ActiveModel { type Error = Error; }


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
            Self::UserTokenId => t!("entities.user_token.user_token_id.name"),
            Self::Token => t!("entities.user_token.token.name"),
            Self::PublicUserId => t!("entities.user_token.public_user_id.name"),
            Self::AccessToken => t!("entities.user_token.access_token.name"),
            Self::RefreshToken => t!("entities.user_token.refresh_token.name"),
            Self::CreatedAt => t!("entities.user_token.created_at.name"),
            Self::ExpiredAt => t!("entities.user_token.expired_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("UserToken: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("UserToken: Database error.")]
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
pub enum Relation {}
