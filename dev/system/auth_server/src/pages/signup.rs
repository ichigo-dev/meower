//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, I18n };
use meower_entity::Validate;
use meower_entity::user::ActiveModel as ActiveUser;
use meower_entity::user_auth::ActiveModel as ActiveUserAuth;
use meower_entity::user_account::ActiveModel as ActiveUserAccount;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Redirect, IntoResponse };
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::{ ActiveValue, TransactionTrait };


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct SignupForm
{
    user_account_name: String,
    email: String,
    email_confirm: String,
    password: String,
    password_confirm: String,
}


//------------------------------------------------------------------------------
/// Signup page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate
{
    i18n: I18n,
    input: SignupForm,
    errors: Vec<String>,
}

impl Default for SignupTemplate
{
    fn default() -> Self
    {
        Self
        {
            i18n: I18n::new(),
            input: SignupForm::default(),
            errors: Vec::new(),
        }
    }
}


//------------------------------------------------------------------------------
/// Signup success page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "signup_success.html")]
pub(crate) struct SignupSuccessTemplate
{
    i18n: I18n,
}

impl Default for SignupSuccessTemplate
{
    fn default() -> Self
    {
        Self
        {
            i18n: I18n::new(),
        }
    }
}


//------------------------------------------------------------------------------
/// Handles signup page.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    Extension(auth): Extension<Auth>,
    Extension(i18n): Extension<I18n>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    if auth.is_logined().await
    {
        return Err(Redirect::to("/"));
    }

    let template = SignupTemplate
    {
        i18n,
        ..Default::default()
    };
    Ok(Html(template.render().unwrap()))
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Form(input): Form<SignupForm>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let hdb = state.hdb();

    // Checks if the email and password confirmations match.
    if input.email != input.email_confirm
    {
        let errors = vec!
        [
            i18n.get("auth_server.signup.form.error.email_not_match")
        ];
        let template = SignupTemplate { i18n, input, errors };
        return Err(Html(template.render().unwrap()));
    }
    if input.password != input.password_confirm
    {
        let errors = vec!
        [
            i18n.get("auth_server.signup.form.error.password_not_match")
        ];
        let template = SignupTemplate { i18n, input, errors };
        return Err(Html(template.render().unwrap()));
    }

    // Creates a new user and account.
    match create_user(&hdb, &input, &i18n).await
    {
        Ok(_) =>
        {
            let template = SignupSuccessTemplate { i18n };
            return Ok(Html(template.render().unwrap()));
        },
        Err(e) =>
        {
            let template = SignupTemplate { i18n, input, errors: vec![e] };
            return Err(Html(template.render().unwrap()));
        },
    };
}


//------------------------------------------------------------------------------
/// Creates a new user and account.
//------------------------------------------------------------------------------
async fn create_user
(
    hdb: &DbConn,
    input: &SignupForm,
    i18n: &I18n,
) -> Result<(), String>
{
    let tsx = hdb.begin().await.unwrap();

    // Creates a user.
    let user: ActiveUser = ActiveUser
    {
        email: ActiveValue::Set(input.email.clone()),
        ..Default::default()
    };
    let user = match user.validate_and_save(&tsx, &i18n).await
    {
        Ok(user) => user,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            return Err(e);
        }
    };

    // Creates a user_auth.
    let user_auth: ActiveUserAuth = ActiveUserAuth
    {
        user_id: user.user_id.clone(),
        password: ActiveValue::Set(input.password.clone()),
        ..Default::default()
    };
    if let Err(e) = user_auth.validate_and_save(&tsx, &i18n).await
    {
        tsx.rollback().await.unwrap();
        return Err(e.to_string());
    }

    // Creates a user_account.
    let user_account: ActiveUserAccount = ActiveUserAccount
    {
        user_id: user.user_id,
        user_account_name: ActiveValue::Set(input.user_account_name.clone()),
        display_name: ActiveValue::Set(input.user_account_name.clone()),
        ..Default::default()
    };
    if let Err(e) = user_account.validate_and_save(&tsx, &i18n).await
    {
        tsx.rollback().await.unwrap();
        return Err(e.to_string());
    };

    tsx.commit().await.unwrap();
    Ok(())
}
