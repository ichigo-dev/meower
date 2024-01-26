//------------------------------------------------------------------------------
//! ClientApplicationAllowOrigin model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::{ Validator, ValidationError };

use async_trait::async_trait;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "client_application_allow_origin")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub client_application_allow_origin_id: i64,
    pub client_application_id: i64,
    pub allow_origin: String,
}


//------------------------------------------------------------------------------
/// ActiveModel.
//------------------------------------------------------------------------------
#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[async_trait]
impl ValidateExt for ActiveModel
{
    type Error = Error;

    //--------------------------------------------------------------------------
    /// Validates the data.
    //--------------------------------------------------------------------------
    async fn validate<C>( &self, _hdb: &C ) -> Result<(), Self::Error>
    where
        C: ConnectionTrait,
    {
        let allow_origin = self
            .allow_origin
            .clone()
            .take()
            .unwrap_or("".to_string());

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .min_length(4)
            .max_length(255)
            .is_url()
            .validate(&allow_origin)
        {
            return Err
            (
                Error::Validation { column: Column::AllowOrigin, error: e }
            );
        }

        Ok(())
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
            Self::ClientApplicationAllowOriginId => t!("entities.client_application_allow_origin.client_application_allow_origin_id.name"),
            Self::ClientApplicationId => t!("entities.client_application_allow_origin.client_application_id.name"),
            Self::AllowOrigin => t!("entities.client_application.allow_origin.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("ClientApplicationAllowOrigin: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("ClientApplicationAllowOrigin: Database error.")]
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
        belongs_to = "super::client_application::Entity",
        from = "Column::ClientApplicationId",
        to = "super::client_application::Column::ClientApplicationId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ClientApplication,
}

impl Related<super::client_application::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::ClientApplication.def()
    }
}
