//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use crate::{ AppState, I18n, Config };
use crate::pages::verify_code::VerifyCodeTemplate;
use meower_entity::Validate;
use meower_entity::temporary_user::ActiveModel as ActiveTemporaryUser;
use meower_entity::temporary_user_code::ActiveModel as ActiveTemporaryUserCode;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, IntoResponse };
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
    email: String,
    email_confirm: String,
    password: String,
    password_confirm: String,
}


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "signup.html")]
struct SignupTemplate
{
    i18n: I18n,
    input: SignupForm,
    errors: Vec<String>,
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
    let template = SignupTemplate
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
    Form(input): Form<SignupForm>,
) -> impl IntoResponse
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
        return Html(template.render().unwrap());
    }
    if input.password != input.password_confirm
    {
        let errors = vec!
        [
            i18n.get("auth_server.signup.form.error.password_not_match")
        ];
        let template = SignupTemplate { i18n, input, errors };
        return Html(template.render().unwrap());
    }

    // Creates a temporary user.
    let tsx = hdb.begin().await.unwrap();
    let token = match create_temporary_user(&tsx, &input, &i18n, &config).await
    {
        Ok(token) => token,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = SignupTemplate { i18n, input, errors: vec![e] };
            return Html(template.render().unwrap());
        }
    };

    tsx.commit().await.unwrap();
    let template = VerifyCodeTemplate
    {
        i18n,
        token,
        errors: Vec::new()
    };
    Html(template.render().unwrap())
}


//------------------------------------------------------------------------------
/// Creates a new temporary user.
//------------------------------------------------------------------------------
pub(crate) async fn create_temporary_user<C>
(
    hdb: &C,
    input: &SignupForm,
    i18n: &I18n,
    config: &Config,
) -> Result<String, String>
where
    C: ConnectionTrait,
{
    // Creates a temporary_user.
    let temporary_user = ActiveTemporaryUser
    {
        email: ActiveValue::Set(input.email.clone()),
        password: ActiveValue::Set(input.password.clone()),
        ..Default::default()
    };
    let temporary_user = match temporary_user
        .validate_and_insert(hdb, &i18n)
        .await
    {
        Ok(temporary_user) => temporary_user,
        Err(e) => return Err(e),
    };

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
