//------------------------------------------------------------------------------
//! Account profile query.
//------------------------------------------------------------------------------

use crate::protect;
use meower_account_entity::account_profile::Column as AccountProfileColumn;
use meower_account_entity::account_profile::Entity as AccountProfileEntity;
use meower_account_entity::account_profile::Model as AccountProfileModel;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, Result };
use sea_orm::{
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
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
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn account_profile
    (
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> Result<AccountProfileModel>
    {
        // Protects the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let account_profile = match protect::check_user_account_profile
        (
            tsx,
            &token,
            &jwt_claims.public_user_id,
        ).await
        {
            Ok((_, account_profile)) => account_profile,
            Err(e) => return Err(e.into()),
        };

        Ok(account_profile)
    }

    //--------------------------------------------------------------------------
    /// Gets account profiles of the logged in user.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn account_profiles
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<Vec<AccountProfileModel>>
    {
        // Protects the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let account = match protect::check_user_account
        (
            tsx,
            &account_name,
            &jwt_claims.public_user_id,
        ).await
        {
            Ok(account) => account,
            Err(e) => return Err(e.into()),
        };

        let account_profiles = AccountProfileEntity::find()
            .filter(AccountProfileColumn::AccountId.eq(account.account_id))
            .all(tsx)
            .await
            .unwrap();
        Ok(account_profiles)
    }
}
