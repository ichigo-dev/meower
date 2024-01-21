//------------------------------------------------------------------------------
//! Verify code page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::pages::signup_success::PageTemplate as SignupSuccessPageTemplate;

use meower_auth_entity::temporary_user::Column as TemporaryUserColumn;
use meower_auth_entity::temporary_user::Entity as TemporaryUserEntity;

use askama::Template;
use axum::extract::{ Form, State };
use axum::response::{ Html, IntoResponse };
use rust_i18n::t;
use serde::Deserialize;
use sea_orm::{
    ColumnTrait,
    EntityTrait,
    ModelTrait,
    QueryFilter,
    TransactionTrait,
};


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "verify_code.html")]
pub(crate) struct PageTemplate
{
    pub(crate) input: FormData,
    pub(crate) token: String,
    pub(crate) input_error: FormError,
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------

// FormData
#[derive(Deserialize, Debug, Default)]
pub(crate) struct FormData
{
    pub(crate) code: String,
    pub(crate) token: String,
}

// Error
#[derive(Default)]
pub(crate) struct FormError
{
    pub(crate) code: Option<String>,
    pub(crate) other: Option<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Form(input): Form<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let tsx = state.hdb.begin().await.unwrap();

    // Finds the temporary user.
    let temporary_user = match TemporaryUserEntity::find()
            .filter(TemporaryUserColumn::Token.eq(&input.token))
            .one(&tsx)
            .await
            .unwrap()
    {
        Some(temporary_user) => temporary_user,
        None =>
        {
            let template = PageTemplate
            {
                input,
                input_error: FormError
                {
                    code: Some(t!("pages.verify_code.error.invalid_url")),
                    ..Default::default()
                },
                ..Default::default()
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Verifies the code.
    if let Err(e) = temporary_user.verify_code(&tsx, &input.code).await
    {
        let template = PageTemplate
        {
            input,
            input_error: FormError
            {
                code: Some(e.get_message()),
                ..Default::default()
            },
            ..Default::default()
        };
        return Err(Html(template.render().unwrap()));
    };

    // Registers the user.
    if let Err(e) = temporary_user.register(&tsx).await
    {
        let template = PageTemplate
        {
            input,
            input_error: FormError
            {
                other: Some(e.get_message()),
                ..Default::default()
            },
            ..Default::default()
        };
        return Err(Html(template.render().unwrap()));
    };

    // Deletes the temporary user.
    if temporary_user.delete(&tsx).await.is_err()
    {
        tsx.rollback().await.unwrap();
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                other: Some(t!("system.error")),
                ..Default::default()
            },
            ..Default::default()
        };
        return Err(Html(template.render().unwrap()));
    }

    tsx.commit().await.unwrap();
    let template = SignupSuccessPageTemplate::default();
    return Ok(Html(template.render().unwrap()));
}
