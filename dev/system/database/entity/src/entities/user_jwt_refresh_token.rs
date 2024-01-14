//------------------------------------------------------------------------------
//! UserJwtRefreshToken model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;
use crate::utils::token;

use async_trait::async_trait;
use chrono::{ Utc, Duration };
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;

const TOKEN_EXPIRATION_MINUTES: i64 = 10;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("UserJwtRefreshToken: Database error.")]
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
    pub fn get_column_name( &self ) -> String
    {
        match self
        {
            Self::UserJwtRefreshTokenId => t!("entities.user_jwt_refresh_token.user_jwt_refresh_token_id.name"),
            Self::Token => t!("entities.user_jwt_refresh_token.token.name"),
            Self::CreatedAt => t!("entities.user_jwt_refresh_token.created_at.name"),
            Self::ExpiredAt => t!("entities.user_jwt_refresh_token.expired_at.name"),
        }
        .to_string()
    }
}


//------------------------------------------------------------------------------
/// Entity.
//------------------------------------------------------------------------------
impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds user_jwt_refresh_token by token.
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
#[sea_orm(table_name = "user_jwt_refresh_token")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_jwt_refresh_token_id: i64,
    #[sea_orm(unique)]
    pub token: String,
    pub created_at: DateTime,
    pub expired_at: DateTime,
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

            let expired_at = now + Duration::minutes(TOKEN_EXPIRATION_MINUTES);
            self.set(Column::ExpiredAt, expired_at.into());

            let token = token::generate_token(None);
            self.set(Column::Token, token.into());
        }

        Ok(self)
    }
}

impl ValidateExt for ActiveModel { type Error = Error; }


//------------------------------------------------------------------------------
/// Relation.
//------------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
