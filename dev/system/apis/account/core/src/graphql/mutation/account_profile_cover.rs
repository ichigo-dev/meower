//------------------------------------------------------------------------------
//! AccountProfileCover mutation.
//------------------------------------------------------------------------------

use crate::Config;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account_profile::Column as AccountProfileColumn;
use meower_account_entity::account_profile::Entity as AccountProfileEntity;
use meower_account_entity::account_profile_cover::ActiveModel as AccountProfileCoverActiveModel;
use meower_account_entity::account_profile_cover::Entity as AccountProfileCoverEntity;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use base64::prelude::*;
use object_store::ObjectStore;
use object_store::path::Path as StoragePath;
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    ModelTrait,
    QueryFilter,
};


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject, Debug)]
struct UploadCoverInput
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
pub(crate) struct AccountProfileCoverMutation;

#[Object]
impl AccountProfileCoverMutation
{
    //--------------------------------------------------------------------------
    /// Uploads cover.
    //--------------------------------------------------------------------------
    async fn upload_cover
    (
        &self,
        ctx: &Context<'_>,
        input: UploadCoverInput,
    ) -> Result<bool>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let config = ctx.data::<Config>().unwrap();
        let storage = ctx.data::<Arc<Box<dyn ObjectStore>>>().unwrap().as_ref();

        let account_profile = match AccountProfileEntity::find()
            .filter(AccountProfileColumn::Token.eq(input.account_profile_token))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account_profile) => account_profile,
            None => return Err(t!("system.error.not_found").into()),
        };

        let account = match account_profile
            .find_related(AccountEntity)
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

        if input.base64.is_some() || input.delete_file
        {
            if let Some(exists_cover) = account_profile
                .find_related(AccountProfileCoverEntity)
                .one(tsx)
                .await
                .unwrap()
            {
                let exists_cover_path = StoragePath::from
                (
                    format!
                    (
                        "{}/{}",
                        config.cover_path,
                        exists_cover.file_key
                    )
                );
                storage.delete(&exists_cover_path).await.unwrap();
                exists_cover.delete(tsx).await.unwrap();
            };
        }

        if let Some(base64) = input.base64
        {
            let (prefix, base64) = match base64.split_once(",")
            {
                Some((content_type, base64)) => (content_type, base64),
                None => return Err(t!("system.error.invalid_format").into()),
            };
            let content_type = prefix
                .split(";")
                .next()
                .unwrap()
                .split(":")
                .last()
                .unwrap();
            let binary = BASE64_STANDARD.decode(base64.as_bytes()).unwrap();
            let file_len = binary.len().try_into().unwrap_or_default();

            if content_type.starts_with("image/") == false
            {
                return Err(t!("system.error.invalid_format").into());
            }

            let account_profile_id = account_profile.account_profile_id;
            let cover = AccountProfileCoverActiveModel
            {
                account_profile_id: ActiveValue::Set(account_profile_id),
                content_type: ActiveValue::Set(content_type.to_string()),
                file_name: ActiveValue::Set(input.file_name.unwrap_or_default()),
                file_size: ActiveValue::Set(file_len),
                ..Default::default()
            };
            let cover = match cover.validate_and_insert(tsx).await
            {
                Ok(cover) => cover,
                Err(e) => return Err(e.get_message().into()),
            };

            let cover_path = StoragePath::from
            (
                format!("{}/{}", config.cover_path, cover.file_key)
            );
            if let Err(e) = storage.put(&cover_path, binary.into()).await
            {
                cover.delete(tsx).await.unwrap();
                return Err(e.to_string().into());
            }
        }

        Ok(true)
    }
}
