//------------------------------------------------------------------------------
//! Account query.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::entity_linked::AccountToGroup;
use meower_account_entity::group::Column as GroupColumn;
use meower_account_entity::group::Entity as GroupEntity;
use meower_account_entity::group::Model as GroupModel;
use meower_account_entity::group_member::Column as GroupMemberColumn;
use meower_account_entity::group_member::Entity as GroupMemberEntity;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, Result };
use casbin::{ CoreApi, Enforcer };
use rust_i18n::t;
use sea_orm::{
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
    ModelTrait,
};
use tokio::sync::RwLock;


//------------------------------------------------------------------------------
/// Query.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct GroupQuery;

#[Object]
impl GroupQuery
{
    //--------------------------------------------------------------------------
    /// Gets group.
    //--------------------------------------------------------------------------
    async fn group
    (
        &self,
        ctx: &Context<'_>,
        group_name: String,
    ) -> Result<GroupModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let group = match GroupEntity::find()
            .filter(GroupColumn::GroupName.eq(&group_name))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(group) => group,
            None => return Err(t!("system.error.not_found").into()),
        };

        if group.is_public
        {
            return Ok(group);
        }

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
            .enforce((&group_member_id, &group_id, "group", "read"))
            .unwrap();
        if result == false
        {
            return Err(t!("system.error.unauthorized").into());
        }

        Ok(group)
    }

    //--------------------------------------------------------------------------
    /// Gets groups.
    //--------------------------------------------------------------------------
    async fn groups
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<Vec<GroupModel>>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account = match AccountEntity::find()
            .filter(AccountColumn::AccountName.eq(&account_name))
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

        let groups = account
            .find_linked(AccountToGroup)
            .all(tsx)
            .await
            .unwrap();
        Ok(groups)
    }
}
