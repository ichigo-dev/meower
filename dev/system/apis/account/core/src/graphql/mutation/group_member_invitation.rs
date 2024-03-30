//------------------------------------------------------------------------------
//! GroupMemberInvitation mutation.
//------------------------------------------------------------------------------

use crate::protect;
use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::group_member_invitation::ActiveModel as GroupMemberInvitationActiveModel;
use meower_account_entity::group_member_invitation::Model as GroupMemberInvitationModel;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use casbin::Enforcer;
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
    selected_account_name: String,
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
        // Protects the access.
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        let enforcer = ctx.data::<Arc<RwLock<Enforcer>>>().unwrap();
        let mut enforcer = enforcer.write().await;
        let group = match protect::enforce_group_member
        (
            tsx,
            &mut enforcer,
            &input.group_name,
            &input.selected_account_name,
            &jwt_claims.public_user_id,
            "group_member_invitation",
            "create"
        ).await
        {
            Ok((group, _, _)) => group,
            Err(_) => return Err(t!("system.error.fatal").into()),
        };

        // Checks if the account exists.
        let account = match AccountEntity::find()
            .filter(AccountColumn::AccountName.eq(&input.account_name))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account) => account,
            None => return Err(t!("system.error.not_found").into()),
        };

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
