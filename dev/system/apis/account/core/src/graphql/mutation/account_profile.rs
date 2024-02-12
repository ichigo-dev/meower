//------------------------------------------------------------------------------
//! AccountProfile mutation.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account_profile::ActiveModel as AccountProfileActiveModel;
use meower_account_entity::account_profile::Gender as AccountProfileGender;
use meower_account_entity::account_profile::Model as AccountProfileModel;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
};
use sea_orm::entity::prelude::*;


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject, Debug)]
struct CreateAccountProfileInput
{
    account_name: String,
    name: String,
    affiliation: Option<String>,
    email: String,
    bio: Option<String>,
    birthdate: Option<DateTime>,
    gender: Option<AccountProfileGender>,
}


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountProfileMutation;

#[Object]
impl AccountProfileMutation
{
    //--------------------------------------------------------------------------
    /// Creates account profile.
    //--------------------------------------------------------------------------
    async fn create_account_profile
    (
        &self,
        ctx: &Context<'_>,
        input: CreateAccountProfileInput,
    ) -> Result<AccountProfileModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account = match AccountEntity::find()
            .filter(AccountColumn::AccountName.eq(&input.account_name))
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

        let account_profile = AccountProfileActiveModel
        {
            account_id: ActiveValue::Set(account.account_id),
            name: ActiveValue::Set(input.name),
            affiliation: ActiveValue::Set(input.affiliation),
            email: ActiveValue::Set(input.email),
            bio: ActiveValue::Set(input.bio),
            birthdate: ActiveValue::Set(input.birthdate),
            gender: ActiveValue::Set(input.gender),
            ..Default::default()
        };
        let account_profile = match account_profile
            .validate_and_insert(tsx)
            .await
        {
            Ok(account_profile) => account_profile,
            Err(e) => return Err(e.get_message().into()),
        };
        Ok(account_profile)
    }
}
