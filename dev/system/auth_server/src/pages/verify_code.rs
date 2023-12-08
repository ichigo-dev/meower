//------------------------------------------------------------------------------
//! Verify code page.
//------------------------------------------------------------------------------

use crate::{ AppState, I18n };
use crate::pages::signup_success::SignupSuccessTemplate;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::temporary_user_code::Model as TemporaryUserCodeModel;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, IntoResponse };
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::TransactionTrait;


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct VerifyCodeForm
{
    token: String,
    code: String,
}


//------------------------------------------------------------------------------
/// Verify code page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "verify_code.html")]
pub(crate) struct VerifyCodeTemplate
{
    pub(crate) i18n: I18n,
    pub(crate) token: String,
    pub(crate) errors: Vec<String>,
}

impl Default for VerifyCodeTemplate
{
    fn default() -> Self
    {
        Self
        {
            i18n: I18n::new(),
            token: String::new(),
            errors: Vec::new(),
        }
    }
}


//------------------------------------------------------------------------------
/// Handles signup page.
//------------------------------------------------------------------------------

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Form(input): Form<VerifyCodeForm>,
) -> impl IntoResponse
{
    let hdb = state.hdb();
    let tsx = hdb.begin().await.unwrap();

    // Create a user.
    if let Err(e) = create_user(&tsx, &input, &i18n).await
    {
        tsx.rollback().await.unwrap();
        let template = VerifyCodeTemplate
        {
            i18n,
            token: input.token,
            errors: vec![e],
        };
        return Html(template.render().unwrap());
    }

    tsx.commit().await.unwrap();
    let template = SignupSuccessTemplate { i18n };
    return Html(template.render().unwrap());
}


//------------------------------------------------------------------------------
/// Creates a new user.
//------------------------------------------------------------------------------
async fn create_user<C>
(
    hdb: &C,
    input: &VerifyCodeForm,
    i18n: &I18n,
) -> Result<(), String>
where
    C: ConnectionTrait,
{
    let temporary_user_code =
        match TemporaryUserCodeModel::find_by_token(hdb, &input.token).await
    {
        Some(temporary_user_code) => temporary_user_code,
        None =>
        {
            return Err(i18n.get("auth_server.verify_code.error.invalid_url"));
        },
    };

    // Verifies a code.
    if temporary_user_code.verify_code(&input.code) == false
    {
        return Err(i18n.get("auth_server.verify_code.error.code_not_match"));
    }

    // Finds a temporary_user.
    let temporary_user = match temporary_user_code
        .find_related(TemporaryUserEntity)
        .one(hdb)
        .await
        .unwrap()
    {
        Some(temporary_user) => temporary_user,
        None =>
        {
            return Err(i18n.get("auth_server.verify_code.error.not_found"));
        }
    };

    // Creates a new user.
    if let Err(e) = temporary_user.create_user(hdb, &i18n).await
    {
        return Err(e);
    }

    // Deletes temporary datas.
    if let Err(e) = temporary_user_code.delete(hdb).await
    {
        return Err(e.to_string());
    }
    if let Err(e) = temporary_user.delete(hdb).await
    {
        return Err(e.to_string());
    }

    Ok(())
}
