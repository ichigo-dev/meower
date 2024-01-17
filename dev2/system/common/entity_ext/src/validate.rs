//------------------------------------------------------------------------------
//! ValidateExt trait.
//!
//!
//! # Required methods
//!
//! - `validate`: Validates the active model.
//------------------------------------------------------------------------------

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;


//------------------------------------------------------------------------------
/// ValidateExt.
//------------------------------------------------------------------------------
#[async_trait]
pub trait ValidateExt: ActiveModelTrait + ActiveModelBehavior + Sync
{
    type Error: From<DbErr> + std::error::Error + Send + Sync + 'static;

    //--------------------------------------------------------------------------
    /// Validates the active model.
    //--------------------------------------------------------------------------
    async fn validate<C>( &self, _hdb: &C ) -> Result<(), Self::Error>
    where
        C: ConnectionTrait
    {
        Ok(())
    }

    //--------------------------------------------------------------------------
    /// Validates and saves the active model.
    //--------------------------------------------------------------------------
    async fn validate_and_save<C>
    (
        self,
        hdb: &C,
    ) -> Result<Self, Self::Error>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
        C: ConnectionTrait,
    {
        self.validate(hdb).await?;
        match self.save(hdb).await
        {
            Ok(model) => Ok(model),
            Err(e) => Err(e.into()),
        }
    }

    //--------------------------------------------------------------------------
    /// Validates and inserts the active model.
    //--------------------------------------------------------------------------
    async fn validate_and_insert<C>
    (
        self,
        hdb: &C,
    ) -> Result<<Self::Entity as EntityTrait>::Model, Self::Error>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
        C: ConnectionTrait,
    {
        self.validate(hdb).await?;
        match self.insert(hdb).await
        {
            Ok(model) => Ok(model),
            Err(e) => Err(e.into()),
        }
    }

    //--------------------------------------------------------------------------
    /// Validates and updates the active model.
    //--------------------------------------------------------------------------
    async fn validate_and_update<C>
    (
        self,
        hdb: &C,
    ) -> Result<<Self::Entity as EntityTrait>::Model, Self::Error>
    where
        <Self::Entity as EntityTrait>::Model: IntoActiveModel<Self>,
        C: ConnectionTrait,
    {
        self.validate(hdb).await?;
        match self.update(hdb).await
        {
            Ok(model) => Ok(model),
            Err(e) => Err(e.into()),
        }
    }
}
