//------------------------------------------------------------------------------
//! Validation utility.
//------------------------------------------------------------------------------

use meower_core::I18n;

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;


//------------------------------------------------------------------------------
/// Validatable.
//------------------------------------------------------------------------------
#[async_trait]
pub trait Validate: ActiveModelTrait + ActiveModelBehavior
{
    //--------------------------------------------------------------------------
    /// Validates the data.
    //--------------------------------------------------------------------------
    async fn validate<C>
    (
        &self,
        hdb: &C,
        i18n: &I18n,
    ) -> Result<(), String>
    where
        C: ConnectionTrait;

    //--------------------------------------------------------------------------
    /// Validates and saves the data.
    //--------------------------------------------------------------------------
    async fn validate_and_save<C>
    (
        self,
        hdb: &C,
        i18n: &I18n,
    ) -> Result<Self, String>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
        C: ConnectionTrait,
    {
        self.validate(hdb, i18n).await?;
        match self.save(hdb).await
        {
            Ok(this) => Ok(this),
            Err(e) => Err(e.to_string()),
        }
    }

    //--------------------------------------------------------------------------
    /// Validates and insert the data.
    //--------------------------------------------------------------------------
    async fn validate_and_insert<C>
    (
        self,
        hdb: &C,
        i18n: &I18n,
    ) -> Result<<Self::Entity as EntityTrait>::Model, String>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
        C: ConnectionTrait,
    {
        self.validate(hdb, i18n).await?;
        match self.insert(hdb).await
        {
            Ok(this) => Ok(this),
            Err(e) => Err(e.to_string()),
        }
    }

    //--------------------------------------------------------------------------
    /// Validates and update the data.
    //--------------------------------------------------------------------------
    async fn validate_and_update<C>
    (
        self,
        hdb: &C,
        i18n: &I18n,
    ) -> Result<<Self::Entity as EntityTrait>::Model, String>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
        C: ConnectionTrait,
    {
        self.validate(hdb, i18n).await?;
        match self.update(hdb).await
        {
            Ok(this) => Ok(this),
            Err(e) => Err(e.to_string()),
        }
    }
}
