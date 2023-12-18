//------------------------------------------------------------------------------
//! Forgot password page.
//------------------------------------------------------------------------------

use crate::{ AppState, I18n, Config };
use meower_entity::user::Entity as UserEntity;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;

use askama::Template;
use axum::Extension;
use axum::response::Html;
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::TransactionTrait;


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct ForgotPasswordForm
{
    email: String,
}


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "forgot_password.html")]
pub(crate) struct ForgotPasswordTemplate
{
    pub(crate) i18n: I18n,
    pub(crate) input: ForgotPasswordForm,
    pub(crate) temporary_user_token: Option<String>,
    pub(crate) deleted_temporary_user: bool,
    pub(crate) errors: Vec<String>,
}

#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "send_reset_password_mail.html")]
pub(crate) struct SendResetPasswordMailTemplate
{
    pub(crate) i18n: I18n,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    Extension(i18n): Extension<I18n>,
) -> Html<String>
{
    let template = ForgotPasswordTemplate
    {
        i18n,
        ..Default::default()
    };
    Html(template.render().unwrap())
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Form(input): Form<ForgotPasswordForm>,
) -> Html<String>
{
    let hdb = state.hdb();
    let config = state.config();

    // Finds the temporary_user.
    let tsx = hdb.begin().await.unwrap();
    let email = input.email.clone();
    if let Some(temporary_user) = TemporaryUserEntity::find_by_email(&email)
        .one(hdb)
        .await
        .unwrap()
    {
        tsx.rollback().await.unwrap();
        let error = i18n.get
        (
            "auth_server.forgot_password.form.error.user_not_verified"
        );
        let template = ForgotPasswordTemplate
        {
            i18n,
            input,
            temporary_user_token: Some(temporary_user.token),
            errors: vec![error],
            ..Default::default()
        };
        return Html(template.render().unwrap());
    }

    // Sends the reset password mail.
    if let Err(e) = reset_password(&tsx, &input, &i18n, &config).await
    {
        tsx.rollback().await.unwrap();
        let template = ForgotPasswordTemplate
        {
            i18n,
            input,
            errors: vec![e],
            ..Default::default()
        };
        return Html(template.render().unwrap());
    }

    tsx.commit().await.unwrap();
    let template = SendResetPasswordMailTemplate
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
    input: &ForgotPasswordForm,
    i18n: &I18n,
    config: &Config,
) -> Result<(), String>
where
    C: ConnectionTrait,
{
    // Finds the user.
    let user = match UserEntity::find_by_email(&input.email)
        .one(hdb)
        .await
        .unwrap()
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

    // Sends the reset password mail.
    if let Err(e) = user.send_reset_password_mail(hdb, &config, &i18n).await
    {
        return Err(e);
    }

    Ok(())
}
