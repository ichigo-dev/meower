//------------------------------------------------------------------------------
//! GroupAvatar model.
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
#[sea_orm(table_name = "group_avatar")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub group_avatar_id: i64,
    pub group_id: i64,
    #[sea_orm(unique)]
    pub file_key: String,
    pub file_name: String,
    pub file_size: i64,
    pub content_type: String,
    pub created_at: DateTime,
}

#[Object(name = "GroupAvatar")]
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
            Self::GroupAvatarId => t!("entities.group_avatar.group_avatar_id.name"),
            Self::GroupId => t!("entities.group_avatar.group_id.name"),
            Self::FileKey => t!("entities.group_avatar.file_key.name"),
            Self::FileName => t!("entities.group_avatar.file_name.name"),
            Self::FileSize => t!("entities.group_avatar.file_size.name"),
            Self::ContentType => t!("entities.group_avatar.content_type.name"),
            Self::CreatedAt => t!("entities.group_avatar.created_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("GroupAvatar: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("GroupAvatar: Database error.")]
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
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::GroupId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Group,
}

impl Related<super::group::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::Group.def()
    }
}
