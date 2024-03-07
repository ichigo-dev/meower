//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::group::Model as GroupModel;
use meower_account_entity::entity_linked::AccountToGroup;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, Result };
use rust_i18n::t;
use sea_orm::{
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
    ModelTrait,
};


//------------------------------------------------------------------------------
/// Query.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct GroupQuery;

#[Object]
impl GroupQuery
{
    //--------------------------------------------------------------------------
    /// Gets groups.
    //--------------------------------------------------------------------------
    async fn groups
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<Vec<GroupModel>>
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

        let groups = account
            .find_linked(AccountToGroup)
            .all(tsx)
            .await
            .unwrap();
        Ok(groups)
    }
}
