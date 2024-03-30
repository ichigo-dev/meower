//------------------------------------------------------------------------------
//! GroupAvatar mutation.
//------------------------------------------------------------------------------

use crate::{ Config, protect, utils };
use meower_account_entity::group_avatar::ActiveModel as GroupAvatarActiveModel;
use meower_account_entity::group_avatar::Entity as GroupAvatarEntity;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use casbin::{ CoreApi, Enforcer };
use object_store::ObjectStore;
use object_store::path::Path as StoragePath;
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    DatabaseTransaction,
    ModelTrait,
};
use tokio::sync::RwLock;


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject, Debug)]
struct UploadGroupAvatarInput
{
    account_name: String,
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

        // Protects the access.
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let enforcer = ctx.data::<Arc<RwLock<Enforcer>>>().unwrap();
        let mut enforcer = enforcer.write().await;
        let (group, group_member) = match protect::enforce_group_member
        (
            tsx,
            &mut enforcer,
            &input.group_name,
            &input.account_name,
            &jwt_claims.public_user_id,
            "group_avatar",
            "create",
        ).await
        {
            Ok((group, _, group_member)) => (group, group_member),
            Err(e) => return Err(e.into()),
        };

        // Deletes the existing avatar.
        if input.base64.is_some() || input.delete_file
        {
            if let Some(exists_avatar) = group
                .find_related(GroupAvatarEntity)
                .one(tsx)
                .await
                .unwrap()
            {
                let result = enforcer
                    .enforce(
                    (
                        &group_member.group_member_id,
                        &group.group_id,
                        "group_avatar",
                        "delete",
                    ))
                    .unwrap();
                if result == false
                {
                    return Err(t!("system.error.unauthorized").into());
                }

                let exists_avatar_path = StoragePath::from
                (
                    format!
                    (
                        "{}/{}",
                        config.group_avatar_path,
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

            let group_id = group.group_id;
            let avatar = GroupAvatarActiveModel
            {
                group_id: ActiveValue::Set(group_id),
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
                format!("{}/{}", config.group_avatar_path, avatar.file_key)
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
