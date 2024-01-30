//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use crate::state::AppState;

use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account::Model as AccountModel;

use async_graphql::{ Context, Object };
use sea_orm::EntityTrait;


//------------------------------------------------------------------------------
/// Account query.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountQuery;

#[Object]
impl AccountQuery
{
    //--------------------------------------------------------------------------
    /// Gets all accounts.
    //--------------------------------------------------------------------------
    async fn accounts( &self, ctx: &Context<'_> ) -> Vec<AccountModel>
    {
        println!("AccountQuery::accounts");
        let state = ctx.data::<AppState>().unwrap();
        let hdb = &state.hdb;
        AccountEntity::find()
            .all(hdb)
            .await
            .unwrap()
    }
}
