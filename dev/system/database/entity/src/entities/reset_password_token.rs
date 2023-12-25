//------------------------------------------------------------------------------
//! ResetPasswordToken model.
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
    #[error("ResetPasswordToken: The token is expired.")]
    TokenExpired,

    #[error("ResetPasswordToken: Database error.")]
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
            Self::TokenExpired =>
            {
                return
                (
                    Some(Column::Token),
                    t!("entities.reset_password_token.token.error.expired"),
                );
            },
            Self::DbError(_) => (None, t!("common.error.db")),
        }
    }
}


//------------------------------------------------------------------------------
/// Entity.
//------------------------------------------------------------------------------
impl Entity
{
    //--------------------------------------------------------------------------
    /// Finds temporary_user_code by token.
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
#[sea_orm(table_name = "reset_password_token")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub reset_password_token_id: i64,
    pub user_id: i64,
    #[sea_orm(unique)]
    pub token: String,
    pub created_at: DateTime,
    pub expired_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Checks if the token is valid.
    //--------------------------------------------------------------------------
    pub fn is_valid_token( &self ) -> Result<(), Error>
    {
        if Utc::now().naive_utc() > self.expired_at
        {
            return Err(Error::TokenExpired);
        }
        Ok(())
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
        // Deletes the old tokens.
        if insert
        {
            let user_id = self.user_id.clone().unwrap();
            Entity::delete_many()
                .filter(Column::UserId.eq(user_id))
                .exec(hdb)
                .await?;
        }

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
