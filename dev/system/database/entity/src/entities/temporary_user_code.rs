//------------------------------------------------------------------------------
//! TemporaryUserCode model.
//------------------------------------------------------------------------------

use crate::traits::validate::ValidateExt;
use crate::utils::code;

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
    #[error("TemporaryUserCode: The code is expired.")]
    CodeExpired,

    #[error("TemporaryUserCode: The code is not match.")]
    CodeNotMatch,

    #[error("TemporaryUserCode: Database error.")]
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
            Self::CodeExpired =>
            {
                return
                (
                    Some(Column::Code),
                    t!("entities.temporary_user_code.code.error.expired"),
                );
            },
            Self::CodeNotMatch =>
            {
                return
                (
                    Some(Column::Code),
                    t!("entities.temporary_user_code.code.error.not_match"),
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
            Self::TemporaryUserCodeId => t!("entities.temporary_user_code.temporary_user_code_id.name"),
            Self::TemporaryUserId => t!("entities.temporary_user_code.temporary_user_id.name"),
            Self::Code => t!("entities.temporary_user_code.code.name"),
            Self::CreatedAt => t!("entities.temporary_user_code.created_at.name"),
            Self::ExpiredAt => t!("entities.temporary_user_code.expired_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "temporary_user_code")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub temporary_user_code_id: i64,
    #[sea_orm(unique)]
    pub temporary_user_id: i64,
    pub code: String,
    pub created_at: DateTime,
    pub expired_at: DateTime,
}

impl Model
{
    //--------------------------------------------------------------------------
    /// Checks if the code is valid.
    //--------------------------------------------------------------------------
    pub fn is_valid_code( &self, code: &str ) -> Result<(), Error>
    {
        if Utc::now().naive_utc() > self.expired_at
        {
            return Err(Error::CodeExpired);
        }
        if &self.code != code
        {
            return Err(Error::CodeNotMatch);
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
        // Deletes the old codes.
        if insert
        {
            let temporary_user_id = self.temporary_user_id.clone().unwrap();
            Entity::delete_many()
                .filter(Column::TemporaryUserId.eq(temporary_user_id))
                .exec(hdb)
                .await?;
        }

        // Generates and sets a code.
        let code = code::generate_code(None);
        self.set(Column::Code, code.into());

        // Sets the default values.
        if insert
        {
            let now = Utc::now().naive_utc();
            self.set(Column::CreatedAt, now.into());

            let expired_at = now + Duration::minutes(CODE_EXPIRATION_MINUTES);
            self.set(Column::ExpiredAt, expired_at.into());
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
        belongs_to = "super::temporary_user::Entity",
        from = "Column::TemporaryUserId",
        to = "super::temporary_user::Column::TemporaryUserId",
        on_update = "NoAction",
        on_delete = "NoAction"
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
