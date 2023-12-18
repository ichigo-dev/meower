//------------------------------------------------------------------------------
//! Reset password page.
//------------------------------------------------------------------------------

use crate::{ AppState, Config, I18n };
use meower_entity::Validate;
use meower_entity::user::Entity as UserEntity;
use meower_entity::user_auth::ActiveModel as ActiveUserAuth;
use meower_entity::reset_password_token::Entity as ResetPasswordTokenEntity;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Redirect };
use axum::extract::{ State, Form, Path };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::{ ActiveValue, TransactionTrait };


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct ResetPasswordForm
{
    password: String,
    password_confirm: String,
}


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "reset_password.html")]
struct ResetPasswordTemplate
{
    i18n: I18n,
    input: ResetPasswordForm,
    errors: Vec<String>,
}

#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "reset_password_success.html")]
pub(crate) struct ResetPasswordSuccessTemplate
{
    pub(crate) i18n: I18n,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Path(token): Path<String>,
) -> Result<Html<String>, Redirect>
{
    // Finds the reset password token.
    let hdb = state.hdb();
    if ResetPasswordTokenEntity::find_by_token(&token)
        .one(hdb)
        .await
        .unwrap()
        .is_none()
    {
        return Err(Redirect::to("/login"));
    }

    let template = ResetPasswordTemplate
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
    Path(token): Path<String>,
    Form(input): Form<ResetPasswordForm>,
) -> Html<String>
{
    let hdb = state.hdb();
    let config = state.config();

    // Resets the password.
    let tsx = hdb.begin().await.unwrap();
    if let Err(e) = reset_password(&tsx, &input, &config, &i18n, &token).await
    {
        tsx.rollback().await.unwrap();
        let template = ResetPasswordTemplate
        {
            i18n,
            input,
            errors: vec![e],
            ..Default::default()
        };
        return Html(template.render().unwrap());
    }

    tsx.commit().await.unwrap();
    let template = ResetPasswordSuccessTemplate
    {
        i18n,
        ..Default::default()
    };
    Html(template.render().unwrap())
}


//------------------------------------------------------------------------------
/// Resets the password.
//------------------------------------------------------------------------------
pub(crate) async fn reset_password<C>
(
    hdb: &C,
    input: &ResetPasswordForm,
    config: &Config,
    i18n: &I18n,
    token: &str,
) -> Result<(), String>
where
    C: ConnectionTrait,
{
    // Checks the password.
    if input.password != input.password_confirm
    {
        return Err
        (
            i18n.get("auth_server.signup.form.error.password_not_match")
        );
    }

    // Finds and deletes the reset password token.
    let reset_password_token =
        match ResetPasswordTokenEntity::find_by_token(token)
            .one(hdb)
            .await
            .unwrap()
    {
        Some(reset_password_token) => reset_password_token,
        None =>
        {
            let error = i18n.get
            (
                "auth_server.forgot_password.form.error.token_not_found"
            );
            return Err(error);
        },
    };

    // Checks the token.
    if let Err(e) = reset_password_token.is_valid(&config, &i18n)
    {
        return Err(e);
    }

    let user_id = reset_password_token.user_id;
    if let Err(e) = reset_password_token.delete(hdb).await
    {
        return Err(e.to_string());
    }

    // Finds the user.
    let user = match UserEntity::find_by_id(user_id).one(hdb).await.unwrap()
    {
        Some(user) => user,
        None =>
        {
            let error = i18n.get
            (
                "auth_server.forgot_password.form.error.email_not_found"
            );
            return Err(error);
        },
    };

    // Creates a new user_auth.
    let user_auth = ActiveUserAuth
    {
        user_id: ActiveValue::Set(user.user_id),
        password: ActiveValue::Set(input.password.clone()),
        ..Default::default()
    };
    if let Err(e) = user_auth.validate_and_insert(hdb, &i18n).await
    {
        return Err(e);
    };

    Ok(())
}
