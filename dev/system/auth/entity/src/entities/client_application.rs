//------------------------------------------------------------------------------
//! ClientApplication model.
//------------------------------------------------------------------------------

use meower_entity_ext::ValidateExt;
use meower_validator::{ Validator, ValidationError };

use async_trait::async_trait;
use chrono::Utc;
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use thiserror::Error;


//------------------------------------------------------------------------------
/// Model.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "client_application")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub client_application_id: i64,
    pub name: String,
    #[sea_orm(unique)]
    pub client_id: String,
    #[sea_orm(unique)]
    pub client_secret: String,
    pub redirect_uri: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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
        let now = Utc::now().naive_utc();
        if insert
        {
            let client_id = meower_utility::get_random_code(32);
            self.set(Column::ClientId, client_id.into());

            let client_secret = meower_utility::get_random_code(64);
            self.set(Column::ClientSecret, client_secret.into());

            self.set(Column::CreatedAt, now.into());
        }
        self.set(Column::UpdatedAt, now.into());

        Ok(self)
    }
}

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
        let name = self.name.clone().take().unwrap_or("".to_string());
        let redirect_uri = self
            .redirect_uri
            .clone()
            .take()
            .unwrap_or("".to_string());

        // Validates fields.
        if let Err(e) = Validator::new()
            .required()
            .min_length(4)
            .max_length(255)
            .validate(&name)
        {
            return Err(Error::Validation { column: Column::Name, error: e });
        }

        if let Err(e) = Validator::new()
            .required()
            .min_length(4)
            .max_length(255)
            .is_url()
            .validate(&redirect_uri)
        {
            return Err
            (
                Error::Validation { column: Column::RedirectUri, error: e }
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
            Self::ClientApplicationId => t!("entities.client_application.client_application_id.name"),
            Self::Name => t!("entities.client_application.name.name"),
            Self::ClientId => t!("entities.client_application.client_id.name"),
            Self::ClientSecret => t!("entities.client_application.client_secret.name"),
            Self::RedirectUri => t!("entities.client_application.redirect_uri.name"),
            Self::CreatedAt => t!("entities.client_application.created_at.name"),
            Self::UpdatedAt => t!("entities.client_application.updated_at.name"),
        }
    }
}


//------------------------------------------------------------------------------
/// Error.
//------------------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum Error
{
    #[error("ClientApplication: {column:?} {error:?}")]
    Validation { column: Column, error: ValidationError },

    #[error("ClientApplication: Database error.")]
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
