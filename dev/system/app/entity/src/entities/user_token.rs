//------------------------------------------------------------------------------
//! UserToken model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


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
    pub access_token: String,
    pub refresh_token: String,
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
            let token = meower_utility::get_random_str(64);
            self.set(Column::Token, token.into());

            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());
        }

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
            Self::AccessToken => t!("entities.user_token.access_token.name"),
            Self::RefreshToken => t!("entities.user_token.refresh_token.name"),
            Self::CreatedAt => t!("entities.user_token.created_at.name"),
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
