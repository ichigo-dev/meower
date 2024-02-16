//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account::Model as AccountModel;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, Result };
use rust_i18n::t;
use sea_orm::{
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
    QueryOrder,
};


//------------------------------------------------------------------------------
/// Query.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountQuery;

#[Object]
impl AccountQuery
{
    //--------------------------------------------------------------------------
    /// Gets a default account.
    //--------------------------------------------------------------------------
    async fn default_account
    (
        &self,
        ctx: &Context<'_>,
        public_user_id: String,
    ) -> Result<Option<AccountModel>>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let account = AccountEntity::find()
            .order_by_desc(AccountColumn::LastLoginAt)
            .one(tsx)
            .await
            .unwrap();
        Ok(account)
    }

    //--------------------------------------------------------------------------
    /// Gets an account.
    //--------------------------------------------------------------------------
    async fn account
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<Option<AccountModel>>
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

        let account = AccountEntity::find_by_id(account.account_id)
            .one(tsx)
            .await
            .unwrap();
        Ok(account)
    }

    //--------------------------------------------------------------------------
    /// Gets accounts.
    //--------------------------------------------------------------------------
    async fn accounts
    (
        &self,
        ctx: &Context<'_>,
        public_user_id: String,
    ) -> Result<Vec<AccountModel>>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let accounts = AccountEntity::find()
            .filter(AccountColumn::PublicUserId.eq(&public_user_id))
            .all(tsx)
            .await
            .unwrap();
        Ok(accounts)
    }
}
