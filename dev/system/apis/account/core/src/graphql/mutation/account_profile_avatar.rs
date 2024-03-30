//------------------------------------------------------------------------------
//! AccountProfileAvatar mutation.
//------------------------------------------------------------------------------

use crate::{ Config, protect, utils };
use meower_account_entity::account_profile_avatar::ActiveModel as AccountProfileAvatarActiveModel;
use meower_account_entity::account_profile_avatar::Entity as AccountProfileAvatarEntity;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use object_store::ObjectStore;
use object_store::path::Path as StoragePath;
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    DatabaseTransaction,
    ModelTrait,
};


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject, Debug)]
struct UploadAvatarInput
{
    account_profile_token: String,
    file_name: Option<String>,
    base64: Option<String>,
    delete_file: bool,
}


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountProfileAvatarMutation;

#[Object]
impl AccountProfileAvatarMutation
{
    //--------------------------------------------------------------------------
    /// Uploads the avatar of the account profile. The existing avatar will be
    /// deleted.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn upload_avatar
    (
        &self,
        ctx: &Context<'_>,
        input: UploadAvatarInput,
    ) -> Result<bool>
    {
        // Protects the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let account_profile = match protect::check_user_account_profile
        (
            tsx,
            &input.account_profile_token,
            &jwt_claims.public_user_id,
        ).await
        {
            Ok((_, account_profile)) => account_profile,
            Err(e) => return Err(e.into()),
        };

        // Deletes the existing avatar.
        let config = ctx.data::<Config>().unwrap();
        let storage = ctx.data::<Arc<Box<dyn ObjectStore>>>().unwrap().as_ref();
        if input.base64.is_some() || input.delete_file
        {
            if let Some(exists_avatar) = account_profile
                .find_related(AccountProfileAvatarEntity)
                .one(tsx)
                .await
                .unwrap()
            {
                let exists_avatar_path = StoragePath::from
                (
                    format!
                    (
                        "{}/{}",
                        config.avatar_path,
                        exists_avatar.file_key
                    )
                );
                storage.delete(&exists_avatar_path).await.unwrap();
                exists_avatar.delete(tsx).await.unwrap();
            };
        }

        // Uploads the new avatar.
        if let Some(base64) = input.base64
        {
            let base64_content = match utils::parse_base64(&base64)
            {
                Ok(base64_content) => base64_content,
                Err(e) => return Err(e.into()),
            };

            if base64_content.content_type.starts_with("image/") == false
            {
                return Err(t!("system.error.invalid_format").into());
            }

            let account_profile_id = account_profile.account_profile_id;
            let avatar = AccountProfileAvatarActiveModel
            {
                account_profile_id: ActiveValue::Set(account_profile_id),
                content_type: ActiveValue::Set(base64_content.content_type),
                file_name: ActiveValue::Set(input.file_name.unwrap_or_default()),
                file_size: ActiveValue::Set(base64_content.file_size),
                ..Default::default()
            };
            let avatar = match avatar.validate_and_insert(tsx).await
            {
                Ok(avatar) => avatar,
                Err(e) => return Err(e.get_message().into()),
            };

            let avatar_path = StoragePath::from
            (
                format!("{}/{}", config.avatar_path, avatar.file_key)
            );
            if let Err(e) = storage
                .put(&avatar_path, base64_content.binary.into())
                .await
            {
                avatar.delete(tsx).await.unwrap();
                return Err(e.to_string().into());
            }
        }
        Ok(true)
    }
}
