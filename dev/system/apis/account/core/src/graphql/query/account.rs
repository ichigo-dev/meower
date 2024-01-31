//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account::Model as AccountModel;
use meower_shared::JwtClaims;

use async_graphql::{ Context, Object };
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
    /// Gets logined user accounts.
    //--------------------------------------------------------------------------
    async fn get_logged_in_user_accounts
    (
        &self,
        ctx: &Context<'_>,
    ) -> Vec<AccountModel>
    {
        let hdb = ctx.data::<DbConn>().unwrap();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        AccountEntity::find()
            .filter(AccountColumn::UserSubject.eq(&jwt_claims.sub))
            .all(hdb)
            .await
            .unwrap()
    }
}
