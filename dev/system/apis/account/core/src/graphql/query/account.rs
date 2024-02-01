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
    /// Gets accounts.
    //--------------------------------------------------------------------------
    async fn accounts
    (
        &self,
        ctx: &Context<'_>,
        public_user_id: String,
    ) -> Vec<AccountModel>
    {
        let hdb = ctx.data::<DbConn>().unwrap();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != public_user_id
        {
            return vec![];
        }

        AccountEntity::find()
            .filter(AccountColumn::UserPublicId.eq(&public_user_id))
            .all(hdb)
            .await
            .unwrap()
    }
}
