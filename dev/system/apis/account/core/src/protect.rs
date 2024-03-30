//------------------------------------------------------------------------------
//! Protection Utilities.
//------------------------------------------------------------------------------

use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account::Model as AccountModel;
use meower_account_entity::account_profile::Column as AccountProfileColumn;
use meower_account_entity::account_profile::Entity as AccountProfileEntity;
use meower_account_entity::account_profile::Model as AccountProfileModel;
use meower_account_entity::group::Column as GroupColumn;
use meower_account_entity::group::Entity as GroupEntity;
use meower_account_entity::group::Model as GroupModel;
use meower_account_entity::group_member::Column as GroupMemberColumn;
use meower_account_entity::group_member::Entity as GroupMemberEntity;
use meower_account_entity::group_member::Model as GroupMemberModel;

use casbin::{ CoreApi, Enforcer, Filter };
use rust_i18n::t;
use sea_orm::{
    ColumnTrait,
    ConnectionTrait,
    EntityTrait,
    ModelTrait,
    QueryFilter,
};


//------------------------------------------------------------------------------
/// Checks if the current user is the account owner.
//------------------------------------------------------------------------------
pub async fn check_user_account<C>
(
    hdb: &C,
    account_name: &str,
    public_user_id: &str,
) -> Result<AccountModel, String>
where
    C: ConnectionTrait,
{
    let account = match AccountEntity::find()
        .filter(AccountColumn::AccountName.eq(account_name))
        .one(hdb)
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return Err(t!("system.error.not_found").into()),
    };

    if account.public_user_id != public_user_id
    {
        return Err(t!("system.error.not_found").into());
    }

    Ok(account)
}

//------------------------------------------------------------------------------
/// Checks if the current user is the account profile owner.
//------------------------------------------------------------------------------
pub async fn check_user_account_profile<C>
(
    hdb: &C,
    token: &str,
    public_user_id: &str,
) -> Result<(AccountModel, AccountProfileModel), String>
where
    C: ConnectionTrait,
{
    let account_profile = match AccountProfileEntity::find()
        .filter(AccountProfileColumn::Token.eq(token))
        .one(hdb)
        .await
        .unwrap()
    {
        Some(account_profile) => account_profile,
        None => return Err(t!("system.error.not_found").into()),
    };

    let account = match account_profile.find_related(AccountEntity)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return Err(t!("system.error.not_found").into()),
    };

    if account.public_user_id != public_user_id
    {
        return Err(t!("system.error.unauthorized").into());
    }

    Ok((account, account_profile))
}


//------------------------------------------------------------------------------
/// Checks if the current user is the account profile owner.
//------------------------------------------------------------------------------
pub async fn enforce_group_member<C>
(
    hdb: &C,
    enforcer: &mut Enforcer,
    group_name: &str,
    account_name: &str,
    public_user_id: &str,
    object: &str,
    action: &str,
) -> Result<(GroupModel, AccountModel, GroupMemberModel), String>
where
    C: ConnectionTrait,
{
    let group = match GroupEntity::find()
        .filter(GroupColumn::GroupName.eq(group_name))
        .one(hdb)
        .await
        .unwrap()
    {
        Some(group) => group,
        None => return Err(t!("system.error.not_found").into()),
    };

    let account = match AccountEntity::find()
        .filter(AccountColumn::AccountName.eq(account_name))
        .one(hdb)
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return Err(t!("system.error.not_found").into()),
    };

    if public_user_id != account.public_user_id
    {
        return Err(t!("system.error.unauthorized").into());
    }

    let group_member = match GroupMemberEntity::find()
        .filter(GroupMemberColumn::GroupId.eq(group.group_id))
        .filter(GroupMemberColumn::AccountId.eq(account.account_id))
        .one(hdb)
        .await
        .unwrap()
    {
        Some(group_member) => group_member,
        None => return Err(t!("system.error.unauthorized").into()),
    };

    let group_member_id = group_member.group_member_id.to_string();
    let group_id = group.group_id.to_string();
    let _ = enforcer.load_filtered_policy
    (
        Filter { p: vec![], g: vec![&group_member_id, &group_id] }
    );
    let result = enforcer
        .enforce((&group_member_id, &group_id, object, action))
        .unwrap();
    if result == false
    {
        return Err(t!("system.error.unauthorized").into());
    }

    Ok((group, account, group_member))
}
