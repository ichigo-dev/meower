//------------------------------------------------------------------------------
//! AccountProfile mutation.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account_profile::Column as AccountProfileColumn;
use meower_account_entity::account_profile::Entity as AccountProfileEntity;
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
    location: Option<String>,
    email: Option<String>,
    telno: Option<String>,
    bio: Option<String>,
    birthdate: Option<DateTime>,
    gender: Option<AccountProfileGender>,
}

#[derive(InputObject, Debug)]
struct UpdateAccountProfileInput
{
    token: String,
    name: String,
    affiliation: Option<String>,
    location: Option<String>,
    email: Option<String>,
    telno: Option<String>,
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
            location: ActiveValue::Set(input.location),
            email: ActiveValue::Set(input.email),
            telno: ActiveValue::Set(input.telno),
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

    //--------------------------------------------------------------------------
    /// Updates account profile.
    //--------------------------------------------------------------------------
    async fn update_account_profile
    (
        &self,
        ctx: &Context<'_>,
        input: UpdateAccountProfileInput,
    ) -> Result<AccountProfileModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account_profile = match AccountProfileEntity::find()
            .filter(AccountProfileColumn::Token.eq(&input.token))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account_profile) => account_profile,
            None => return Err(t!("system.error.not_found").into()),
        };

        let account = match account_profile.find_related(AccountEntity)
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

        let mut account_profile: AccountProfileActiveModel
            = account_profile.into();
        account_profile.name = ActiveValue::Set(input.name);
        account_profile.affiliation = ActiveValue::Set(input.affiliation);
        account_profile.location = ActiveValue::Set(input.location);
        account_profile.email = ActiveValue::Set(input.email);
        account_profile.telno = ActiveValue::Set(input.telno);
        account_profile.bio = ActiveValue::Set(input.bio);
        account_profile.birthdate = ActiveValue::Set(input.birthdate);
        let account_profile = match account_profile
            .validate_and_update(tsx)
            .await
        {
            Ok(account_profile) => account_profile,
            Err(e) => return Err(e.get_message().into()),
        };

        Ok(account_profile)
    }

    //--------------------------------------------------------------------------
    /// Deletes account profile.
    //--------------------------------------------------------------------------
    async fn delete_account_profile
    (
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> Result<bool>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account_profile = match AccountProfileEntity::find()
            .filter(AccountProfileColumn::Token.eq(&token))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account_profile) => account_profile,
            None => return Err(t!("system.error.not_found").into()),
        };

        let account = match account_profile.find_related(AccountEntity)
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

        if account.default_account_profile_id == account_profile.account_profile_id
        {
            return Err(t!("system.error.fatal").into());
        }

        if let Err(e) = account_profile.delete(tsx).await
        {
            return Err(e.to_string().into());
        };

        Ok(true)
    }
}
