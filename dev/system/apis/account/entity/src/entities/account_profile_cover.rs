//------------------------------------------------------------------------------
//! AccountProfileCover model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::ValidationError;

use async_graphql::Object;
use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "account_profile_cover")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub account_profile_cover_id: i64,
    pub account_profile_id: i64,
    #[sea_orm(unique)]
    pub file_key: String,
    pub file_name: String,
    pub file_size: i64,
    pub content_type: String,
    pub created_at: DateTime,
}

#[Object(name = "AccountProfileCover")]
impl Model
{
    //--------------------------------------------------------------------------
    /// Gets the file key.
    //--------------------------------------------------------------------------
    pub async fn file_key( &self ) -> String
    {
        self.file_key.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the file size.
    //--------------------------------------------------------------------------
    pub async fn file_size( &self ) -> i64
    {
        self.file_size
    }

    //--------------------------------------------------------------------------
    /// Gets the content type.
    //--------------------------------------------------------------------------
    pub async fn content_type( &self ) -> String
    {
        self.content_type.clone()
    }

    //--------------------------------------------------------------------------
    /// Gets the create date.
    //--------------------------------------------------------------------------
    pub async fn created_at( &self ) -> DateTime
    {
        self.created_at
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

            let file_key = meower_utility::get_random_str(128);
            self.set(Column::FileKey, file_key.into());
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
            Self::AccountProfileCoverId => t!("entities.account_profile_cover.account_profile_cover_id.name"),
            Self::AccountProfileId => t!("entities.account_profile_cover.account_profile_id.name"),
            Self::FileKey => t!("entities.account_profile_cover.file_key.name"),
            Self::FileName => t!("entities.account_profile_cover.file_name.name"),
            Self::FileSize => t!("entities.account_profile_cover.file_size.name"),
            Self::ContentType => t!("entities.account_profile_cover.content_type.name"),
            Self::CreatedAt => t!("entities.account_profile_cover.created_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("AccountProfileCover: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("AccountProfileCover: Database error.")]
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
        belongs_to = "super::account_profile::Entity",
        from = "Column::AccountProfileId",
        to = "super::account_profile::Column::AccountProfileId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    AccountProfile,
}

impl Related<super::account_profile::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::AccountProfile.def()
    }
}
