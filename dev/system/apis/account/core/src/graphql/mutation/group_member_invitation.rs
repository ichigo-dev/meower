//------------------------------------------------------------------------------
//! GroupMemberInvitation mutation.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::group::Column as GroupColumn;
use meower_account_entity::group::Entity as GroupEntity;
use meower_account_entity::group_member::Column as GroupMemberColumn;
use meower_account_entity::group_member::Entity as GroupMemberEntity;
use meower_account_entity::group_member_invitation::ActiveModel as GroupMemberInvitationActiveModel;
use meower_account_entity::group_member_invitation::Model as GroupMemberInvitationModel;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use casbin::{ CoreApi, Enforcer, Filter };
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
};
use tokio::sync::RwLock;


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject, Debug)]
struct InviteGroupMemberInput
{
    group_name: String,
    account_name: String,
}


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct GroupMemberInvitationMutation;

#[Object]
impl GroupMemberInvitationMutation
{
    //--------------------------------------------------------------------------
    /// Invites a group member.
    //--------------------------------------------------------------------------
    async fn invite_group_member
    (
        &self,
        ctx: &Context<'_>,
        input: InviteGroupMemberInput,
    ) -> Result<GroupMemberInvitationModel>
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
        let logined_account = match AccountEntity::find()
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
            .filter(GroupMemberColumn::AccountId.eq(logined_account.account_id))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(group_member) => group_member,
            None => return Err(t!("system.error.not_found").into()),
        };
        let enforcer = ctx.data::<Arc<RwLock<Enforcer>>>().unwrap();
        let mut enforcer = enforcer.write().await;
        let group_member_id = group_member.group_member_id.to_string();
        let group_id = group.group_id.to_string();
        let _ = enforcer.load_filtered_policy
        (
            Filter { p: vec![], g: vec![&group_member_id, &group_id] }
        );
        let result = enforcer
            .enforce
            (
                (
                    &group_member_id,
                    &group_id,
                    "group_member_invitation",
                    "create"
                )
            )
            .unwrap();
        if result == false
        {
            return Err(t!("system.error.unauthorized").into());
        }

        // Creates the invitation.
        let group_member_invitation = GroupMemberInvitationActiveModel
        {
            group_id: ActiveValue::Set(group.group_id),
            account_id: ActiveValue::Set(account.account_id),
            ..Default::default()
        };
        let invitation = match group_member_invitation
            .validate_and_insert(tsx)
            .await
        {
            Ok(invitation) => invitation,
            Err(_) => return Err(t!("system.error.fatal").into()),
        };

        Ok(invitation)
    }
}
