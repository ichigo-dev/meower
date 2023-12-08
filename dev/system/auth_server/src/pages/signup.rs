//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, I18n, Config };
use meower_entity::Validate;
use meower_entity::temporary_user::ActiveModel as ActiveTemporaryUser;
use meower_entity::temporary_user_token::ActiveModel as ActiveTemporaryUserToken;

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
    let config = state.config();

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

    // Creates a temporary user.
    let tsx = hdb.begin().await.unwrap();
    match create_temporary_user(&tsx, &input, &i18n, &config).await
    {
        Ok(_) =>
        {
            tsx.commit().await.unwrap();
            let template = SignupSuccessTemplate { i18n };
            return Ok(Html(template.render().unwrap()));
        },
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = SignupTemplate { i18n, input, errors: vec![e] };
            return Err(Html(template.render().unwrap()));
        },
    }
}


//------------------------------------------------------------------------------
/// Creates a new temporary user.
//------------------------------------------------------------------------------
async fn create_temporary_user<C>
(
    hdb: &C,
    input: &SignupForm,
    i18n: &I18n,
    config: &Config,
) -> Result<(), String>
where
    C: ConnectionTrait,
{
    // Creates a temporary_user.
    let temporary_user = ActiveTemporaryUser
    {
        email: ActiveValue::Set(input.email.clone()),
        password: ActiveValue::Set(input.password.clone()),
        user_account_name: ActiveValue::Set(input.user_account_name.clone()),
        ..Default::default()
    };
    let temporary_user = match temporary_user
        .validate_and_insert(hdb, &i18n)
        .await
    {
        Ok(temporary_user) => temporary_user,
        Err(e) => return Err(e),
    };

    // Creates a temporary_user_token.
    let temporary_user_token = ActiveTemporaryUserToken
    {
        temporary_user_id: ActiveValue::Set(temporary_user.temporary_user_id),
        ..Default::default()
    };
    if let Err(e) = temporary_user_token.save(hdb).await
    {
        return Err(e.to_string());
    }

    // Sends a signup email.
    temporary_user.send_signup_email(hdb, &config, &i18n).await?;

    Ok(())
}
