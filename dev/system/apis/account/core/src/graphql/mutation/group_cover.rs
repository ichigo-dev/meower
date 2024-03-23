//------------------------------------------------------------------------------
//! GroupCover mutation.
//------------------------------------------------------------------------------

use crate::Config;
use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::group::Column as GroupColumn;
use meower_account_entity::group::Entity as GroupEntity;
use meower_account_entity::group_cover::ActiveModel as GroupCoverActiveModel;
use meower_account_entity::group_cover::Entity as GroupCoverEntity;
use meower_account_entity::group_member::Column as GroupMemberColumn;
use meower_account_entity::group_member::Entity as GroupMemberEntity;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use base64::prelude::*;
use casbin::{ CoreApi, Enforcer };
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
use tokio::sync::RwLock;


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject, Debug)]
struct UploadGroupCoverInput
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
pub(crate) struct GroupCoverMutation;

#[Object]
impl GroupCoverMutation
{
    //--------------------------------------------------------------------------
    /// Uploads cover.
    //--------------------------------------------------------------------------
    async fn upload_group_cover
    (
        &self,
        ctx: &Context<'_>,
        input: UploadGroupCoverInput,
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

        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let account = match AccountEntity::find()
            .filter(AccountColumn::PublicUserId.eq(&jwt_claims.public_user_id))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account) => account,
            None => return Err(t!("system.error.not_found").into()),
        };

        let group_member = match GroupMemberEntity::find()
            .filter(GroupMemberColumn::GroupId.eq(group.group_id))
            .filter(GroupMemberColumn::AccountId.eq(account.account_id))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(group_member) => group_member,
            None => return Err(t!("system.error.unauthorized").into()),
        };

        // Checks the access.
        let enforcer = ctx.data::<Arc<RwLock<Enforcer>>>().unwrap();
        let enforcer = enforcer.read().await;
        let group_member_id = group_member.group_member_id.to_string();
        let group_id = group.group_id.to_string();
        let result = enforcer
            .enforce((&group_member_id, &group_id, "group_cover", "create"))
            .unwrap();
        if result == false
        {
            return Err(t!("system.error.unauthorized").into());
        }

        if input.base64.is_some() || input.delete_file
        {
            if let Some(exists_cover) = group
                .find_related(GroupCoverEntity)
                .one(tsx)
                .await
                .unwrap()
            {
                let result = enforcer
                    .enforce
                    (
                        (&group_member_id, &group_id, "group_cover", "delete")
                    )
                    .unwrap();
                if result == false
                {
                    return Err(t!("system.error.unauthorized").into());
                }

                let exists_cover_path = StoragePath::from
                (
                    format!
                    (
                        "{}/{}",
                        config.group_cover_path,
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

            let group_id = group.group_id;
            let cover = GroupCoverActiveModel
            {
                group_id: ActiveValue::Set(group_id),
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
                format!("{}/{}", config.group_cover_path, cover.file_key)
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
