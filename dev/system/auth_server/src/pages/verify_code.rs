//------------------------------------------------------------------------------
//! Verify code page.
//------------------------------------------------------------------------------

use crate::{ AppState, I18n, Config };
use crate::pages::signup_success::SignupSuccessTemplate;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::temporary_user_code::Entity as TemporaryUserCodeEntity;

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
pub(crate) struct VerifyCodeForm
{
    token: String,
    code: String,
}


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "verify_code.html")]
pub(crate) struct VerifyCodeTemplate
{
    pub(crate) i18n: I18n,
    pub(crate) token: String,
    pub(crate) errors: Vec<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Form(input): Form<VerifyCodeForm>,
) -> Html<String>
{
    let hdb = state.hdb();
    let config = state.config();
    let tsx = hdb.begin().await.unwrap();

    // Create a user.
    if let Err(e) = create_user(&tsx, &input, &i18n, &config).await
    {
        tsx.rollback().await.unwrap();
        let template = VerifyCodeTemplate
        {
            i18n,
            token: input.token,
            errors: vec![e],
            ..Default::default()
        };
        return Html(template.render().unwrap());
    }

    tsx.commit().await.unwrap();
    let template = SignupSuccessTemplate { i18n };
    Html(template.render().unwrap())
}


//------------------------------------------------------------------------------
/// Creates a new user.
//------------------------------------------------------------------------------
async fn create_user<C>
(
    hdb: &C,
    input: &VerifyCodeForm,
    i18n: &I18n,
    config: &Config,
) -> Result<(), String>
where
    C: ConnectionTrait,
{
    let temporary_user_code =
        match TemporaryUserCodeEntity::find_by_token(&input.token)
            .one(hdb)
            .await
            .unwrap()
    {
        Some(temporary_user_code) => temporary_user_code,
        None =>
        {
            return Err(i18n.get("auth_server.verify_code.error.invalid_url"));
        },
    };

    // Verifies a code.
    if let Err(e) = temporary_user_code.verify_code(&input.code, &config, &i18n)
    {
        return Err(e);
    };

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
