//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account::Model as AccountModel;
use meower_shared::JwtClaims;

use async_graphql::{ Context, Object, Result };
use rust_i18n::t;
use sea_orm::{ ColumnTrait, DbConn, EntityTrait, QueryFilter };


//------------------------------------------------------------------------------
/// Account query.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountQuery;

#[Object]
impl AccountQuery
{
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
        let hdb = ctx.data::<DbConn>().unwrap();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let accounts = AccountEntity::find()
            .filter(AccountColumn::PublicUserId.eq(&public_user_id))
            .all(hdb)
            .await
            .unwrap();
        Ok(accounts)
    }
}
