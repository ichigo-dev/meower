//------------------------------------------------------------------------------
//! Account profile query.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account_profile::Column as AccountProfileColumn;
use meower_account_entity::account_profile::Entity as AccountProfileEntity;
use meower_account_entity::account_profile::Model as AccountProfileModel;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, Result };
use rust_i18n::t;
use sea_orm::{
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    ModelTrait,
    QueryFilter,
};


//------------------------------------------------------------------------------
/// Query.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountProfileQuery;

#[Object]
impl AccountProfileQuery
{
    //--------------------------------------------------------------------------
    /// Gets account profile.
    //--------------------------------------------------------------------------
    async fn account_profile
    (
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> Result<AccountProfileModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account_profile = match AccountProfileEntity::find()
            .filter(AccountProfileColumn::Token.eq(token))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account_profile) => account_profile,
            None => return Err(t!("system.error.not_found").into()),
        };

        let account = match account_profile
            .find_related(AccountEntity)
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account) => account,
            None => return Err(t!("system.error.not_found").into()),
        };

        if jwt_claims.public_user_id != account.public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        Ok(account_profile)
    }

    //--------------------------------------------------------------------------
    /// Gets account profiles.
    //--------------------------------------------------------------------------
    async fn account_profiles
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<Vec<AccountProfileModel>>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account = match AccountEntity::find()
            .filter(AccountColumn::AccountName.eq(&account_name))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account) => account,
            None => return Err(t!("system.error.not_found").into()),
        };

        if jwt_claims.public_user_id != account.public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let account_profiles = AccountProfileEntity::find()
            .filter(AccountProfileColumn::AccountId.eq(account.account_id))
            .all(tsx)
            .await
            .unwrap();
        Ok(account_profiles)
    }
}
