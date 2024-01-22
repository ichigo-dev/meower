//------------------------------------------------------------------------------
//! Resend verify code page. 
//------------------------------------------------------------------------------

use crate::AppState;
use crate::utils::email::get_mailer;
use crate::pages::verify_code::PageTemplate as VerifyCodePageTemplate;

use meower_auth_entity::temporary_user::Column as TemporaryUserColumn;
use meower_auth_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_auth_entity::temporary_user_code::ActiveModel as TemporaryUserCodeActiveModel;
use meower_entity_ext::ValidateExt;

use askama::Template;
use axum::response::{ Html, IntoResponse };
use axum::extract::{ Form, State };
use lettre::{ AsyncTransport, Message };
use lettre::message::header::ContentType;
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    EntityTrait,
    QueryFilter,
    TransactionTrait,
};
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "resend_verify_code.html")]
pub(crate) struct PageTemplate
{
    pub(crate) input: FormData,
    pub(crate) input_error: FormError,
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------

// FormData
#[derive(Deserialize, Debug, Default)]
pub(crate) struct FormData
{
    pub(crate) email: String,
    pub(crate) password: String,
}

// Error
#[derive(Default)]
pub(crate) struct FormError
{
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) other: Option<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler() -> impl IntoResponse
{
    let template = PageTemplate::default();
    Html(template.render().unwrap())
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Form(input): Form<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config;
    let tsx = state.hdb.begin().await.unwrap();

    // Checks the email is not empty.
    if input.email.is_empty()
    {
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                email: Some(t!("pages.resend_verify_code.form.email.error.required")),
                ..Default::default()
            },
        };
        return Ok(Html(template.render().unwrap()));
    }

    // Checks the password is not empty.
    if input.password.is_empty()
    {
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                password: Some(t!("pages.resend_verify_code.form.password.error.required")),
                ..Default::default()
            },
        };
        return Ok(Html(template.render().unwrap()));
    }

    // Creates a temporary user.
    let temporary_user = match TemporaryUserEntity::find()
        .filter(TemporaryUserColumn::Email.eq(&input.email))
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(temporary_user) => temporary_user,
        None =>
        {
            // Ends normally as a countermeasure against blind SQL injection.
            tsx.rollback().await.unwrap();
            let template = VerifyCodePageTemplate
            {
                token: "".to_string(),
                ..Default::default()
            };
            return Ok(Html(template.render().unwrap()));
        },
    };

    // Verifies the password.
    if temporary_user.verify_password(&input.password) == false
    {
        // Ends normally as a countermeasure against blind SQL injection.
        tsx.rollback().await.unwrap();
        let template = VerifyCodePageTemplate
        {
            token: "".to_string(),
            ..Default::default()
        };
        return Ok(Html(template.render().unwrap()));
    }

    // Creates a temporary user code.
    let temporary_user_code = TemporaryUserCodeActiveModel
    {
        temporary_user_id: ActiveValue::set(temporary_user.temporary_user_id),
        ..Default::default()
    };
    let temporary_user_code = match temporary_user_code
        .validate_and_insert(&tsx)
        .await
    {
        Ok(temporary_user_code) => temporary_user_code,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = PageTemplate
            {
                input: input,
                input_error: FormError
                {
                    other: Some(e.get_message()),
                    ..Default::default()
                }
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Sends a confirmation email.
    let mailer = get_mailer(&config);
    let email = Message::builder()
        .from(config.system_email_address.parse().unwrap())
        .to(input.email.clone().parse().unwrap())
        .header(ContentType::TEXT_HTML)
        .subject(t!("messages.email.resend_verify_code.subject"))
        .body
        (
            t!
            (
                "messages.email.resend_verify_code.body",
                verify_code = temporary_user_code.code
            )
        )
        .unwrap();
    if let Err(e) = mailer.send(email).await
    {
        tsx.rollback().await.unwrap();
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                other: Some(e.to_string()),
                ..Default::default()
            },
        };
        return Err(Html(template.render().unwrap()));
    };

    tsx.commit().await.unwrap();
    let template = VerifyCodePageTemplate
    {
        token: temporary_user.token,
        ..Default::default()
    };
    Ok(Html(template.render().unwrap()))
}
