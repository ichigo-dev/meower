//------------------------------------------------------------------------------
//! Group mutation.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::group::Column as GroupColumn;
use meower_account_entity::group::Entity as GroupEntity;
use meower_account_entity::group::ActiveModel as GroupActiveModel;
use meower_account_entity::group::Model as GroupModel;
use meower_account_entity::group_member::Column as GroupMemberColumn;
use meower_account_entity::group_member::Entity as GroupMemberEntity;
use meower_account_entity::group_member::ActiveModel as GroupMemberActiveModel;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use casbin::{ CoreApi, Enforcer, MgmtApi };
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
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
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn create_group
    (
        &self,
        ctx: &Context<'_>,
        input: CreateGroupInput,
    ) -> Result<GroupModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let account = match AccountEntity::find()
            .filter(AccountColumn::AccountName.eq(&input.account_name))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account) => account,
            None => return Err(t!("system.error.not_found").into()),
        };

        // Protects the access.
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != account.public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

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
        let group_id = group.group_id.to_string();
        enforcer.add_policy(vec!
        [
            "admin".to_string(),
            group.group_id.to_string(),
            format!("group:{}", &group_id),
            "*".to_string(),
        ]).await.unwrap();
        enforcer.add_policy(vec!
        [
            "member".to_string(),
            group.group_id.to_string(),
            format!("group:{}", &group_id),
            "read".to_string(),
        ]).await.unwrap();
        enforcer.add_named_policy("g", vec!
        [
            group_member.group_member_id.to_string(),
            group.group_id.to_string(),
            "admin".to_string(),
        ]).await.unwrap();
        enforcer.save_policy().await.unwrap();

       Ok(group)
    }

    //--------------------------------------------------------------------------
    /// Updates the group.
    ///
    /// * Access is protected from users other than the owner.
    //--------------------------------------------------------------------------
    async fn update_group
    (
        &self,
        ctx: &Context<'_>,
        input: UpdateGroupInput,
    ) -> Result<GroupModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let group = match GroupEntity::find()
            .filter(GroupColumn::GroupName.eq(&input.group_name))
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
        let request =
        (
            &group_member_id,
            &group_id,
            format!("group:{}", &group_id),
            "update",
        );
        if enforcer.enforce(request).unwrap() == false
        {
            return Err(t!("system.error.unauthorized").into());
        }

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
