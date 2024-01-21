//------------------------------------------------------------------------------
//! TemporaryUserCode model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use async_trait::async_trait;
use chrono::{ Duration, Utc };
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;

const CODE_EXPIRATION_MINUTES: i64 = 10;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "temporary_user_code")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub temporary_user_code_id: i64,
    pub temporary_user_id: i64,
    pub code: String,
    pub created_at: DateTime,
    pub expired_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Verifies code.
    //--------------------------------------------------------------------------
    pub fn verify( &self, code: &str ) -> Result<(), Error>
    {
        if self.code != code
        {
            return Err(Error::CodeNotMatch);
        }

        let now = Utc::now().naive_utc();
        if now <= self.expired_at
        {
            return Err(Error::CodeExpired);
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
        // Deletes the old datas.
        if insert
        {
            let temporary_user_id = self
                .temporary_user_id
                .clone()
                .take()
                .unwrap_or(0);
            Entity::delete_many()
                .filter(Column::TemporaryUserId.eq(temporary_user_id))
                .exec(hdb)
                .await?;
        }

        // Sets the default values.
        if insert
        {
            let code = meower_utility::get_random_code(6);
            self.set(Column::Code, code.into());

            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());

            let expired_at = now + Duration::minutes(CODE_EXPIRATION_MINUTES);
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
            Self::TemporaryUserCodeId => t!("entities.temporary_user_code.temporary_user_code_id.name"),
            Self::TemporaryUserId => t!("entities.temporary_user_code.temporary_user_id.name"),
            Self::Code => t!("entities.temporary_user_code.code.name"),
            Self::CreatedAt => t!("entities.temporary_user_code.created_at.name"),
            Self::ExpiredAt => t!("entities.temporary_user_code.expired_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("TemporaryUser: The code is not match.")]
    CodeNotMatch,

    #[error("TemporaryUser: The token is expired.")]
    CodeExpired,

    #[error("TemporaryUser: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

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
            Self::CodeNotMatch => Some(Column::Code),
            Self::CodeExpired => Some(Column::ExpiredAt),
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
            Self::CodeNotMatch =>
            {
                t!("entities.temporary_user_code.code.error.not_match")
            },
            Self::CodeExpired =>
            {
                t!("entities.temporary_user_code.expired_at.error.expired")
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
    #[sea_orm(
        belongs_to = "super::temporary_user::Entity",
        from = "Column::TemporaryUserId",
        to = "super::temporary_user::Column::TemporaryUserId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    TemporaryUser,
}

impl Related<super::temporary_user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::TemporaryUser.def()
    }
}
