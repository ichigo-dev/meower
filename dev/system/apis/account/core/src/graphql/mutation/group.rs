//------------------------------------------------------------------------------
//! Group mutation.
//------------------------------------------------------------------------------

use crate::protect;
use meower_account_entity::group::ActiveModel as GroupActiveModel;
use meower_account_entity::group::Model as GroupModel;
use meower_account_entity::group_member::ActiveModel as GroupMemberActiveModel;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use casbin::{ CoreApi, Enforcer, MgmtApi };
use sea_orm::{
    ActiveValue,
    DatabaseTransaction,
};
use sea_orm::entity::prelude::*;
use tokio::sync::RwLock;


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject, Debug)]
struct CreateGroupInput
{
    account_name: String,
    group_name: String,
    name: String,
    description: Option<String>,
    representative: Option<String>,
    location: Option<String>,
    email: Option<String>,
    telno: Option<String>,
    founded_at: Option<DateTime>,
    is_public: bool,
}

#[derive(InputObject, Debug)]
struct UpdateGroupInput
{
    account_name: String,
    group_name: String,
    name: String,
    description: Option<String>,
    representative: Option<String>,
    location: Option<String>,
    email: Option<String>,
    telno: Option<String>,
    founded_at: Option<DateTime>,
    is_public: bool,
}


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct GroupMutation;

#[Object]
impl GroupMutation
{
    //--------------------------------------------------------------------------
    /// Creates a new group with the specified account as administrator. The
    /// account must belong to the logged in user.
    //--------------------------------------------------------------------------
    async fn create_group
    (
        &self,
        ctx: &Context<'_>,
        input: CreateGroupInput,
    ) -> Result<GroupModel>
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

        // Creates the group.
        let group = GroupActiveModel
        {
            group_name: ActiveValue::Set(input.group_name),
            name: ActiveValue::Set(input.name),
            description: ActiveValue::Set(input.description),
            representative: ActiveValue::Set(input.representative),
            location: ActiveValue::Set(input.location),
            email: ActiveValue::Set(input.email),
            telno: ActiveValue::Set(input.telno),
            founded_at: ActiveValue::Set(input.founded_at),
            is_public: ActiveValue::Set(input.is_public),
            ..Default::default()
        };
        let group = match group.validate_and_insert(tsx).await
        {
            Ok(group) => group,
            Err(e) => return Err(e.get_message().into()),
        };

        // Adds the group member as the administrator.
        let group_id = group.group_id;
        let group_member = GroupMemberActiveModel
        {
            group_id: ActiveValue::Set(group_id),
            account_id: ActiveValue::Set(account.account_id),
            account_profile_id: ActiveValue::Set(account.default_account_profile_id),
            ..Default::default()
        };
        let group_member = match group_member.validate_and_insert(tsx).await
        {
            Ok(group_member) => group_member,
            Err(e) => return Err(e.get_message().into()),
        };

        // Creates the default group policy.
        let enforcer = ctx.data::<Arc<RwLock<Enforcer>>>().unwrap();
        let mut enforcer = enforcer.write().await;
        let _ = enforcer.load_policy();
        let group_objects =
        [
            "group",
            "group_member",
            "group_avatar",
            "group_cover",
            "group_workspace",
        ];

        // Admin role.
        enforcer.add_policy(vec!
        [
            "admin".to_string(),
            group.group_id.to_string(),
            "*".to_string(),
            "*".to_string(),
        ]).await.unwrap();
        enforcer.add_grouping_policy(vec!
        [
            group_member.group_member_id.to_string(),
            group.group_id.to_string(),
            "admin".to_string(),
        ]).await.unwrap();

        // Member role.
        for group_object in group_objects
        {
            enforcer.add_policy(vec!
            [
                "member".to_string(),
                group.group_id.to_string(),
                group_object.to_string(),
                "read".to_string(),
            ]).await.unwrap();
        }
        enforcer.save_policy().await.unwrap();

        Ok(group)
    }

    //--------------------------------------------------------------------------
    /// Updates the group.
    //--------------------------------------------------------------------------
    async fn update_group
    (
        &self,
        ctx: &Context<'_>,
        input: UpdateGroupInput,
    ) -> Result<GroupModel>
    {
        // Checks the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let enforcer = ctx.data::<Arc<RwLock<Enforcer>>>().unwrap();
        let mut enforcer = enforcer.write().await;
        let group = match protect::enforce_group_member
        (
            tsx,
            &mut enforcer,
            &input.group_name,
            &input.account_name,
            &jwt_claims.public_user_id,
            "group",
            "update"
        ).await
        {
            Ok((group, _, _)) => group,
            Err(e) => return Err(e.into()),
        };

        let mut group: GroupActiveModel = group.into();
        group.name = ActiveValue::Set(input.name);
        group.description = ActiveValue::Set(input.description);
        group.representative = ActiveValue::Set(input.representative);
        group.location = ActiveValue::Set(input.location);
        group.email = ActiveValue::Set(input.email);
        group.telno = ActiveValue::Set(input.telno);
        group.founded_at = ActiveValue::Set(input.founded_at);
        group.is_public = ActiveValue::Set(input.is_public);
        let group = match group.validate_and_update(tsx).await
        {
            Ok(group) => group,
            Err(e) => return Err(e.get_message().into()),
        };

        Ok(group)
    }
}
