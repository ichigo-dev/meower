//------------------------------------------------------------------------------
//! Account mutation.
//------------------------------------------------------------------------------

use meower_account_entity::account::ActiveModel as AccountActiveModel;
use meower_account_entity::account::Model as AccountModel;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use rust_i18n::t;
use sea_orm::{ ActiveValue, DatabaseTransaction };


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject)]
struct CreateAccountInput
{
    public_user_id: String,
    account_name: String,
}


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountMutation;

#[Object]
impl AccountMutation
{
    //--------------------------------------------------------------------------
    /// Creates accounts.
    //--------------------------------------------------------------------------
    async fn create_account
    (
        &self,
        ctx: &Context<'_>,
        input: CreateAccountInput,
    ) -> Result<AccountModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != input.public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let account = AccountActiveModel
        {
            public_user_id: ActiveValue::Set(input.public_user_id),
            account_name: ActiveValue::Set(input.account_name),
            ..Default::default()
        };
        let account = match account.validate_and_insert(tsx).await
        {
            Ok(account) => account,
            Err(e) => return Err(e.get_message().into()),
        };
        Ok(account)
    }
}
