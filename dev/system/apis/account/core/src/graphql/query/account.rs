//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use crate::protect;
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
    /// Gets the default account of the logged in user.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn default_account
    (
        &self,
        ctx: &Context<'_>,
        public_user_id: String,
    ) -> Result<Option<AccountModel>>
    {
        // Protects the access.
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let account = AccountEntity::find()
            .order_by_desc(AccountColumn::LastLoginAt)
            .one(tsx)
            .await
            .unwrap();
        Ok(account)
    }

    //--------------------------------------------------------------------------
    /// Gets an account from the account name.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn account
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<Option<AccountModel>>
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

        let account = AccountEntity::find_by_id(account.account_id)
            .one(tsx)
            .await
            .unwrap();
        Ok(account)
    }

    //--------------------------------------------------------------------------
    /// Gets accounts of the logged in user.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn accounts
    (
        &self,
        ctx: &Context<'_>,
        public_user_id: String,
    ) -> Result<Vec<AccountModel>>
    {
        // Protects the access.
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let accounts = AccountEntity::find()
            .filter(AccountColumn::PublicUserId.eq(&public_user_id))
            .order_by_asc(AccountColumn::CreatedAt)
            .all(tsx)
            .await
            .unwrap();
        Ok(accounts)
    }

    //--------------------------------------------------------------------------
    /// Searches for accounts by starting with the account name.
    /// Private accounts are not included.
    //--------------------------------------------------------------------------
    async fn search_accounts
    (
        &self,
        ctx: &Context<'_>,
        search: String,
    ) -> Result<Vec<AccountModel>>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let accounts = AccountEntity::find()
            .filter(AccountColumn::AccountName.like(search + "%"))
            .filter(AccountColumn::IsPublic.eq(true))
            .all(tsx)
            .await
            .unwrap();
        Ok(accounts)
    }
}
