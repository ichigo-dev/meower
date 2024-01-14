//------------------------------------------------------------------------------
//! UserJwtTokenCode model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;
use crate::utils::token;

use async_trait::async_trait;
use chrono::{ Utc, Duration };
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;

const CODE_EXPIRATION_MINUTES: i64 = 10;


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("UserJwtTokenCode: Database error.")]
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
            Self::UserJwtTokenCodeId => t!("entities.user_jwt_token_code.user_jwt_token_code_id.name"),
            Self::UserId => t!("entities.user_jwt_token_code.user_id.name"),
            Self::Code => t!("entities.user_jwt_token_code.code.name"),
            Self::CreatedAt => t!("entities.user_jwt_subject.created_at.name"),
            Self::ExpiredAt => t!("entities.user_jwt_token_code.expired_at.name"),
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
    /// Finds user_jwt_token_code by code.
    //--------------------------------------------------------------------------
    pub fn find_by_code( code: &str ) -> Select<Self>
    {
        Self::find().filter(Column::Code.eq(code))
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_jwt_token_code")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub user_jwt_token_code_id: i64,
    pub user_id: i64,
    #[sea_orm(unique)]
    pub code: String,
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
        hdb: &C,
        insert: bool,
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Deletes the old subjects.
        if insert
        {
            let user_id = self.user_id.clone().take().unwrap_or(0);
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

            let expired_at = now + Duration::minutes(CODE_EXPIRATION_MINUTES);
            self.set(Column::ExpiredAt, expired_at.into());

            let code = token::generate_token(None);
            self.set(Column::Code, code.into());
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
