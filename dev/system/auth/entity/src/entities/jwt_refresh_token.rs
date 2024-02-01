//------------------------------------------------------------------------------
//! JwtRefreshToken model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use async_trait::async_trait;
use chrono::{ Duration, Utc };
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;

const TOKEN_EXPIRATION_HOURS: i64 = 24;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "jwt_refresh_token")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub jwt_refresh_token_id: i64,
    pub user_id: i64,
    #[sea_orm(unique)]
    pub token: String,
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
            let user_id = self
                .user_id
                .clone()
                .take()
                .unwrap_or(0);
            Entity::delete_many()
                .filter(Column::UserId.eq(user_id))
                .exec(hdb)
                .await?;
        }

        // Sets the default values.
        if insert
        {
            let token = meower_utility::get_random_str(128);
            self.set(Column::Token, token.into());

            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());

            let expired_at = now + Duration::hours(TOKEN_EXPIRATION_HOURS);
            self.set(Column::ExpiredAt, expired_at.into());
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
            Self::JwtRefreshTokenId => t!("entities.jwt_refresh_token.jwt_refresh_token_id.name"),
            Self::UserId => t!("entities.jwt_refresh_token.user_id.name"),
            Self::Token => t!("entities.jwt_refresh_token.token.name"),
            Self::CreatedAt => t!("entities.jwt_refresh_token.created_at.name"),
            Self::ExpiredAt => t!("entities.jwt_refresh_token.expired_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("JwtRefreshToken: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("JwtRefreshToken: Database error.")]
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
pub enum Relation
{
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "Cascade"
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
