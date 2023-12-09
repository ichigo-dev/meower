//------------------------------------------------------------------------------
//! Resend verify code page.
//------------------------------------------------------------------------------

use crate::{ AppState, I18n, Config };
use crate::pages::verify_code::VerifyCodeTemplate;
use meower_entity::FieldVerify;
use meower_entity::temporary_user::Model as TemporaryUserModel;
use meower_entity::temporary_user_code::ActiveModel as ActiveTemporaryUserCode;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, IntoResponse };
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::{ TransactionTrait, ActiveValue };


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct ResendVerifyCodeForm
{
    email: String,
    password: String,
}


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "resend_verify_code.html")]
struct ResendVerifyCodeTemplate
{
    pub(crate) i18n: I18n,
    pub(crate) input: ResendVerifyCodeForm,
    pub(crate) errors: Vec<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    Extension(i18n): Extension<I18n>,
) -> impl IntoResponse
{
    let template = ResendVerifyCodeTemplate
    {
        i18n,
        input: ResendVerifyCodeForm::default(),
        errors: Vec::new(),
    };
    return Html(template.render().unwrap());
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Form(input): Form<ResendVerifyCodeForm>,
) -> impl IntoResponse
{
    let hdb = state.hdb();
    let config = state.config();
    let tsx = hdb.begin().await.unwrap();

    // Resend a verify code.
    let token = match resend_verify_code(&tsx, &input, &i18n, &config).await
    {
        Ok(token) => token,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = ResendVerifyCodeTemplate
            {
                i18n,
                input,
                errors: vec![e],
            };
            return Html(template.render().unwrap());
        }
    };

    tsx.commit().await.unwrap();
    let template = VerifyCodeTemplate
    {
        i18n,
        token,
        errors: Vec::new(),
    };
    return Html(template.render().unwrap());
}


//------------------------------------------------------------------------------
/// Resends a verify code.
//------------------------------------------------------------------------------
async fn resend_verify_code<C>
(
    hdb: &C,
    input: &ResendVerifyCodeForm,
    i18n: &I18n,
    config: &Config,
) -> Result<String, String>
where
    C: ConnectionTrait,
{
    // Finds a temporary_user.
    let temporary_user
        = match TemporaryUserModel::find_by_email(hdb, &input.email).await
    {
        Some(temporary_user) => temporary_user,
        None =>
        {
            let error = i18n.get
            (
                "auth_server.resend_verify_code.form.error.user_not_found"
            );
            return Err(error);
        }
    };

    // Verifies a password.
    if temporary_user.verify_field(&input.password) == false
    {
        let error = i18n.get
        (
            "auth_server.resend_verify_code.form.error.invalid_password"
        );
        return Err(error);
    }

    // Creates a temporary_user_code.
    let temporary_user_code = ActiveTemporaryUserCode
    {
        temporary_user_id: ActiveValue::Set(temporary_user.temporary_user_id),
        ..Default::default()
    };
    let temporary_user_code = match temporary_user_code.insert(hdb).await
    {
        Ok(temporary_user_code) => temporary_user_code,
        Err(e) => return Err(e.to_string()),
    };

    // Sends a signup email.
    temporary_user.send_verify_mail(hdb, &config, &i18n).await?;

    let token = temporary_user_code.token.clone();
    Ok(token)
}
