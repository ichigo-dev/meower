//------------------------------------------------------------------------------
//! AccountProfile mutation.
//------------------------------------------------------------------------------

use crate::{ Config, protect };
use meower_account_entity::account_profile::ActiveModel as AccountProfileActiveModel;
use meower_account_entity::account_profile::Gender as AccountProfileGender;
use meower_account_entity::account_profile::Model as AccountProfileModel;
use meower_account_entity::account_profile_avatar::Entity as AccountProfileAvatarEntity;
use meower_account_entity::account_profile_cover::Entity as AccountProfileCoverEntity;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use object_store::ObjectStore;
use object_store::path::Path as StoragePath;
use rust_i18n::t;
use sea_orm::{ ActiveValue, DatabaseTransaction };
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
    /// Creates a new account profile for the logged in user's account.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn create_account_profile
    (
        &self,
        ctx: &Context<'_>,
        input: CreateAccountProfileInput,
    ) -> Result<AccountProfileModel>
    {
        // Protects the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let account = match protect::check_user_account
        (
            tsx,
            &input.account_name,
            &jwt_claims.public_user_id,
        ).await
        {
            Ok(account) => account,
            Err(e) => return Err(e.into()),
        };

        // Creates an account profile.
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
    /// Updates the account profile of the logged in user's account.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn update_account_profile
    (
        &self,
        ctx: &Context<'_>,
        input: UpdateAccountProfileInput,
    ) -> Result<AccountProfileModel>
    {
        // Protects the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let account_profile = match protect::check_user_account_profile
        (
            tsx,
            &input.token,
            &jwt_claims.public_user_id,
        ).await
        {
            Ok((_, account_profile)) => account_profile,
            Err(e) => return Err(e.into()),
        };

        // Updates the account profile.
        let mut account_profile: AccountProfileActiveModel
            = account_profile.into();
        account_profile.name = ActiveValue::Set(input.name);
        account_profile.affiliation = ActiveValue::Set(input.affiliation);
        account_profile.location = ActiveValue::Set(input.location);
        account_profile.email = ActiveValue::Set(input.email);
        account_profile.telno = ActiveValue::Set(input.telno);
        account_profile.bio = ActiveValue::Set(input.bio);
        account_profile.birthdate = ActiveValue::Set(input.birthdate);
        account_profile.gender = ActiveValue::Set(input.gender);
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
    /// Deletes the account profile of the logged in user's account. Also
    /// deletes the avatar and cover.
    /// The default account profile cannot be deleted.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn delete_account_profile
    (
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> Result<bool>
    {
        // Protects the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let (account, account_profile) = match protect::check_user_account_profile
        (
            tsx,
            &token,
            &jwt_claims.public_user_id,
        ).await
        {
            Ok((account, account_profile)) => (account, account_profile),
            Err(e) => return Err(e.into()),
        };

        // The default account profile cannot be deleted.
        let default_account_profile_id = account.default_account_profile_id;
        if default_account_profile_id == account_profile.account_profile_id
        {
            return Err(t!("system.error.fatal").into());
        }

        // Also deletes the avatar and cover.
        let config = ctx.data::<Config>().unwrap();
        let storage = ctx.data::<Arc<Box<dyn ObjectStore>>>().unwrap().as_ref();
        if let Some(avatar) = account_profile
            .find_related(AccountProfileAvatarEntity)
            .one(tsx)
            .await
            .unwrap()
        {
            let path = StoragePath::from
            (
                format!
                (
                    "{}/{}",
                    config.avatar_path,
                    avatar.file_key
                )
            );
            storage.delete(&path).await.unwrap();
        };
        if let Some(cover) = account_profile
            .find_related(AccountProfileCoverEntity)
            .one(tsx)
            .await
            .unwrap()
        {
            let path = StoragePath::from
            (
                format!
                (
                    "{}/{}",
                    config.cover_path,
                    cover.file_key
                )
            );
            storage.delete(&path).await.unwrap();
        };

        if let Err(e) = account_profile.delete(tsx).await
        {
            return Err(e.to_string().into());
        };
        Ok(true)
    }
}
