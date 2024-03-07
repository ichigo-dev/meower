//------------------------------------------------------------------------------
//! GroupAvatar mutation.
//------------------------------------------------------------------------------

use crate::Config;
use meower_account_entity::group::Column as GroupColumn;
use meower_account_entity::group::Entity as GroupEntity;
use meower_account_entity::group_avatar::ActiveModel as GroupAvatarActiveModel;
use meower_account_entity::group_avatar::Entity as GroupAvatarEntity;
use meower_entity_ext::ValidateExt;

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
struct UploadGroupAvatarInput
{
    group_name: String,
    file_name: Option<String>,
    base64: Option<String>,
    delete_file: bool,
}


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct GroupAvatarMutation;

#[Object]
impl GroupAvatarMutation
{
    //--------------------------------------------------------------------------
    /// Uploads avatar.
    //--------------------------------------------------------------------------
    async fn upload_group_avatar
    (
        &self,
        ctx: &Context<'_>,
        input: UploadGroupAvatarInput,
    ) -> Result<bool>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let config = ctx.data::<Config>().unwrap();
        let storage = ctx.data::<Arc<Box<dyn ObjectStore>>>().unwrap().as_ref();

        let group = match GroupEntity::find()
            .filter(GroupColumn::GroupName.eq(input.group_name))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(group) => group,
            None => return Err(t!("system.error.not_found").into()),
        };

        // TODO: Check if the user is the owner of the group.

        if input.base64.is_some() || input.delete_file
        {
            if let Some(exists_avatar) = group
                .find_related(GroupAvatarEntity)
                .one(tsx)
                .await
                .unwrap()
            {
                let exists_avatar_path = StoragePath::from
                (
                    config.group_avatar_path.clone() + "/" +
                    &exists_avatar.file_key
                );
                storage.delete(&exists_avatar_path).await.unwrap();
                exists_avatar.delete(tsx).await.unwrap();
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

            let group_id = group.group_id;
            let avatar = GroupAvatarActiveModel
            {
                group_id: ActiveValue::Set(group_id),
                content_type: ActiveValue::Set(content_type.to_string()),
                file_name: ActiveValue::Set(input.file_name.unwrap_or_default()),
                file_size: ActiveValue::Set(file_len),
                ..Default::default()
            };
            let avatar = match avatar.validate_and_insert(tsx).await
            {
                Ok(avatar) => avatar,
                Err(e) => return Err(e.get_message().into()),
            };

            let avatar_path = StoragePath::from
            (
                config.group_avatar_path.clone() + "/" + &avatar.file_key
            );
            if let Err(e) = storage.put(&avatar_path, binary.into()).await
            {
                avatar.delete(tsx).await.unwrap();
                return Err(e.to_string().into());
            }
        }

        Ok(true)
    }
}
