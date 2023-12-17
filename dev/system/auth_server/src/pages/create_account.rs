//------------------------------------------------------------------------------
//! Create account page.
//------------------------------------------------------------------------------

use crate::{ AppState, JwtClaims, Auth, I18n, Config };
use meower_entity::Validate;
use meower_entity::user::Entity as UserEntity;
use meower_entity::user_account::ActiveModel as ActiveUserAccount;
use meower_entity::user_jwt_subject::Entity as UserJwtSubjectEntity;
use meower_entity::workspace::ActiveModel as ActiveWorkspace;
use meower_entity::user_account_workspace::ActiveModel as ActiveUserAccountWorkspace;
use meower_entity::workspace_member_authority::Entity as WorkspaceMemberAuthorityEntity;
use meower_entity::workspace_member_authority::AuthorityMap as WorkspaceMemberAuthorityMap;
use meower_entity::workspace_member::ActiveModel as ActiveWorkspaceMember;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Response, Redirect, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form, Path };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::{ ActiveValue, TransactionTrait };


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct CreateAccountForm
{
    user_account_name: String,
}


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "create_account.html")]
struct CreateAccountTemplate
{
    i18n: I18n,
    input: CreateAccountForm,
    user_jwt_subject: String,
    errors: Vec<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Path(sub): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let hdb = state.hdb();

    // Finds the user_jwt_subject.
    if UserJwtSubjectEntity::find_by_subject(&sub)
        .one(hdb)
        .await
        .unwrap()
        .is_none()
    {
        return Err(Redirect::to("/auth/login"));
    }

    let template = CreateAccountTemplate
    {
        i18n,
        user_jwt_subject: sub,
        ..Default::default()
    };
    Ok(Html(template.render().unwrap()))
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Path(sub): Path<String>,
    Form(input): Form<CreateAccountForm>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let hdb = state.hdb();
    let config = state.config();

    // Finds the temporary_user.
    let tsx = hdb.begin().await.unwrap();

    // Resend a verify code.
    let jwt_cookie
        = match create_user_account(&tsx, &input, &i18n, &config, &sub).await
    {
        Ok(jwt_cookie) => jwt_cookie,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = CreateAccountTemplate
            {
                i18n,
                input,
                user_jwt_subject: sub,
                errors: vec![e],
                ..Default::default()
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Proxies to the frontend.
    tsx.commit().await.unwrap();
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, jwt_cookie.to_string())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}


//------------------------------------------------------------------------------
/// Creates user_account.
//------------------------------------------------------------------------------
async fn create_user_account<C>
(
    hdb: &C,
    input: &CreateAccountForm,
    i18n: &I18n,
    config: &Config,
    sub: &str,
) -> Result<String, String>
where
    C: ConnectionTrait,
{
    // Finds the user_jwt_subject.
    let user_jwt_subject = match UserJwtSubjectEntity::find_by_subject(sub)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(user_jwt_subject) => user_jwt_subject,
        None =>
        {
            return Err
            (
                i18n.get("auth_server.create_account.error.invalid_url")
            );
        },
    };

    // Finds the logined user.
    let user_id = user_jwt_subject.user_id;
    let user = match UserEntity::find_by_id(user_id)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            return Err
            (
                i18n.get("auth_server.create_account.error.user_not_found")
            );
        },
    };

    // Creates the user account.
    let user_account_name = &input.user_account_name;
    let user_account = ActiveUserAccount
    {
        user_id: ActiveValue::Set(user.user_id),
        user_account_name: ActiveValue::Set(user_account_name.clone()),
        display_name: ActiveValue::Set(user_account_name.clone()),
        ..Default::default()
    };
    let user_account = match user_account.validate_and_insert(hdb, &i18n).await
    {
        Ok(user_account) => user_account,
        Err(e) => return Err(e),
    };

    // Creates the user account profile.
    let user_account_profile = meower_entity::user_account_profile::ActiveModel
    {
        user_account_id: ActiveValue::Set(user_account.user_account_id),
        ..Default::default()
    };
    if let Err(e) = user_account_profile.validate_and_insert(hdb, &i18n).await
    {
        return Err(e);
    }

    // Creates the default workspace.
    let display_name = format!
    (
        "{}'s workspace",
        user_account_name.clone()
    );
    let workspace = ActiveWorkspace
    {
        workspace_name: ActiveValue::Set(user_account_name.clone()),
        display_name: ActiveValue::Set(display_name),
        ..Default::default()
    };
    let workspace = match workspace.validate_and_insert(hdb, &i18n).await
    {
        Ok(workspace) => workspace,
        Err(e) => return Err(e),
    };

    // Creates the user account workspace.
    let user_account_workspace = ActiveUserAccountWorkspace
    {
        user_account_id: ActiveValue::Set(user_account.user_account_id),
        workspace_id: ActiveValue::Set(workspace.workspace_id),
        ..Default::default()
    };
    if let Err(e) = user_account_workspace.validate_and_insert(hdb, &i18n).await
    {
        return Err(e);
    }

    // Creates workspace member.
    let authority = WorkspaceMemberAuthorityEntity::find_by_authority
    (
        &WorkspaceMemberAuthorityMap::Admin
    )
        .one(hdb)
        .await
        .unwrap()
        .unwrap();
    let authority_id = authority.workspace_member_authority_id;
    let workspace_member = ActiveWorkspaceMember
    {
        workspace_id: ActiveValue::Set(workspace.workspace_id),
        user_account_id: ActiveValue::Set(user_account.user_account_id),
        workspace_member_authority_id: ActiveValue::Set(authority_id),
        ..Default::default()
    };
    if let Err(e) = workspace_member.validate_and_insert(hdb, &i18n).await
    {
        return Err(e);
    }

    // Creates a JWT claims.
    let mut claims = JwtClaims::init_from_config(&config);
    claims.sub = user_jwt_subject.subject;
    claims.uan = user_account.user_account_name;
    let auth = Auth::init(claims);
    Ok(auth.make_jwt_cookie(&config))
}
