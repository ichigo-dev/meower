//------------------------------------------------------------------------------
//! Account mutation.
//------------------------------------------------------------------------------

use meower_account_entity::account::ActiveModel as AccountActiveModel;
use meower_account_entity::account::Column as AccountColumn;
use meower_account_entity::account::Entity as AccountEntity;
use meower_account_entity::account::Model as AccountModel;
use meower_account_entity::account_profile::ActiveModel as AccountProfileActiveModel;
use meower_account_entity::account_profile::Column as AccountProfileColumn;
use meower_account_entity::account_profile::Entity as AccountProfileEntity;
use meower_account_entity::account_workspace::ActiveModel as AccountWorkspaceActiveModel;
use meower_account_entity::workspace::ActiveModel as WorkspaceActiveModel;
use meower_account_entity::workspace::Column as WorkspaceColumn;
use meower_account_entity::workspace::Entity as WorkspaceEntity;
use meower_entity_ext::ValidateExt;
use meower_shared::JwtClaims;

use std::sync::Arc;

use async_graphql::{ Context, Object, InputObject, Result };
use chrono::Utc;
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    QueryFilter,
};


//------------------------------------------------------------------------------
/// Inputs.
//------------------------------------------------------------------------------
#[derive(InputObject)]
struct CreateAccountInput
{
    public_user_id: String,
    account_name: String,
    email: String,
    is_public: bool,
}

#[derive(InputObject)]
struct UpdateAccountInput
{
    account_name: String,
    email: Option<String>,
    is_public: Option<bool>,
    default_account_profile_token: Option<String>,
    default_workspace_name: Option<String>,
}


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountMutation;

#[Object]
impl AccountMutation
{
    //--------------------------------------------------------------------------
    /// Select account.
    //--------------------------------------------------------------------------
    async fn select_account
    (
        &self,
        ctx: &Context<'_>,
        account_name: String,
    ) -> Result<AccountModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account = match AccountEntity::find()
            .filter(AccountColumn::AccountName.eq(account_name))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account) => account,
            None => return Err(t!("system.error.not_found").into()),
        };
        if account.public_user_id != jwt_claims.public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let mut account: AccountActiveModel = account.into();
        let now = Utc::now().naive_utc();
        account.last_login_at = ActiveValue::Set(now);
        let account = match account.validate_and_update(tsx).await
        {
            Ok(account) => account,
            Err(e) => return Err(e.get_message().into()),
        };

        Ok(account)
    }

    //--------------------------------------------------------------------------
    /// Creates account.
    //--------------------------------------------------------------------------
    async fn create_account
    (
        &self,
        ctx: &Context<'_>,
        input: CreateAccountInput,
    ) -> Result<AccountModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();
        if jwt_claims.public_user_id != input.public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let account = AccountActiveModel
        {
            public_user_id: ActiveValue::Set(input.public_user_id),
            account_name: ActiveValue::Set(input.account_name),
            email: ActiveValue::Set(input.email),
            default_account_profile_id: ActiveValue::Set(0),
            default_workspace_id: ActiveValue::Set(0),
            is_public: ActiveValue::Set(input.is_public),
            ..Default::default()
        };
        let account = match account.validate_and_insert(tsx).await
        {
            Ok(account) => account,
            Err(e) => return Err(e.get_message().into()),
        };

        let account_profile = AccountProfileActiveModel
        {
            account_id: ActiveValue::Set(account.account_id),
            name: ActiveValue::Set(account.account_name.clone()),
            ..Default::default()
        };
        let account_profile = match account_profile
            .validate_and_insert(tsx)
            .await
        {
            Ok(account_profile) => account_profile,
            Err(e) => return Err(e.get_message().into()),
        };

        let workspace_name = meower_utility::get_random_str(32);
        let name = account.account_name.clone() + "'s workspace";
        let workspace = WorkspaceActiveModel
        {
            workspace_name: ActiveValue::Set(workspace_name),
            name: ActiveValue::Set(name),
            ..Default::default()
        };
        let workspace = match workspace.validate_and_insert(tsx).await
        {
            Ok(workspace) => workspace,
            Err(e) => return Err(e.get_message().into()),
        };

        let account_workspace = AccountWorkspaceActiveModel
        {
            account_id: ActiveValue::Set(account.account_id),
            workspace_id: ActiveValue::Set(workspace.workspace_id),
            ..Default::default()
        };
        if let Err(e) = account_workspace.validate_and_insert(tsx).await
        {
            return Err(e.get_message().into());
        };

        let mut account: AccountActiveModel = account.into();
        account.default_account_profile_id = ActiveValue::Set
        (
            account_profile.account_profile_id
        );
        account.default_workspace_id = ActiveValue::Set(workspace.workspace_id);
        let account = match account.validate_and_update(tsx).await
        {
            Ok(account) => account,
            Err(e) => return Err(e.get_message().into()),
        };

        Ok(account)
    }

    //--------------------------------------------------------------------------
    /// Updates account.
    //--------------------------------------------------------------------------
    async fn update_account
    (
        &self,
        ctx: &Context<'_>,
        input: UpdateAccountInput,
    ) -> Result<AccountModel>
    {
        let tsx = ctx.data::<Arc<DatabaseTransaction>>().unwrap().as_ref();
        let jwt_claims = ctx.data::<JwtClaims>().unwrap();

        let account = match AccountEntity::find()
            .filter(AccountColumn::AccountName.eq(input.account_name))
            .one(tsx)
            .await
            .unwrap()
        {
            Some(account) => account,
            None => return Err(t!("system.error.not_found").into()),
        };
        if account.public_user_id != jwt_claims.public_user_id
        {
            return Err(t!("system.error.unauthorized").into());
        }

        let mut account: AccountActiveModel = account.into();

        if let Some(email) = input.email
        {
            account.email = ActiveValue::Set(email);
        };
        if let Some(is_public) = input.is_public
        {
            account.is_public = ActiveValue::Set(is_public);
        };

        if let Some(token) = input.default_account_profile_token
        {
            let account_profile = match AccountProfileEntity::find()
                .filter(AccountProfileColumn::Token.eq(token))
                .one(tsx)
                .await
                .unwrap()
            {
                Some(account_profile) => account_profile,
                None => return Err(t!("system.error.not_found").into()),
            };

            account.default_account_profile_id
                = ActiveValue::Set(account_profile.account_profile_id);
        };
        if let Some(workspace_name) = input.default_workspace_name
        {
            let workspace = match WorkspaceEntity::find()
                .filter(WorkspaceColumn::WorkspaceName.eq(workspace_name))
                .one(tsx)
                .await
                .unwrap()
            {
                Some(workspace) => workspace,
                None => return Err(t!("system.error.not_found").into()),
            };

            account.default_workspace_id =
                ActiveValue::Set(workspace.workspace_id);
        };
        let account = match account.validate_and_update(tsx).await
        {
            Ok(account) => account,
            Err(e) => return Err(e.get_message().into()),
        };

        Ok(account)
    }
}
