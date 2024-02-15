//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account::Model as AccountModel;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, Result, SimpleObject };
use rust_i18n::t;
use sea_orm::entity::prelude::*;
use sea_orm::{ ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter };


//------------------------------------------------------------------------------
/// Output.
//------------------------------------------------------------------------------
#[derive(SimpleObject)]
struct Account
{
    account_id: i64,
    account_name: String,
    public_user_id: String,
    default_account_profile_id: i64,
    default_workspace_id: i64,
    created_at: DateTime,
    last_login_at: DateTime,
}

impl From<AccountModel> for Account
{
    fn from(model: AccountModel) -> Self
    {
        Account
        {
            account_id: model.account_id,
            account_name: model.account_name,
            public_user_id: model.public_user_id,
            default_account_profile_id: model.default_account_profile_id,
            default_workspace_id: model.default_workspace_id,
            created_at: model.created_at,
            last_login_at: model.last_login_at,
        }
    }
}


//------------------------------------------------------------------------------
/// Query.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountQuery;

#[Object]
impl AccountQuery
{
    //--------------------------------------------------------------------------
    /// Gets an account.
    //--------------------------------------------------------------------------
    async fn account
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<Option<Account>>
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
    ) -> Result<Vec<Account>>
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
