//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::utils::email::get_mailer;
use crate::handlers::verify_code::PageTemplate as VerifyCodePageTemplate;

use meower_auth_entity::temporary_user::Column as TemporaryUserColumn;
use meower_auth_entity::temporary_user::ActiveModel as TemporaryUserActiveModel;
use meower_auth_entity::temporary_user_code::ActiveModel as TemporaryUserCodeActiveModel;
use meower_entity_ext::ValidateExt;

use askama::Template;
use axum::extract::{ Form, State };
use axum::response::{ Html, IntoResponse };
use lettre::AsyncTransport;
use lettre::Message;
use lettre::message::header::ContentType;
use rust_i18n::t;
use serde::Deserialize;
use sea_orm::{ ActiveValue, TransactionTrait };


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[derive(Template, Default)]
#[template(path = "signup.html")]
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
    pub(crate) email_confirm: String,
    pub(crate) password: String,
    pub(crate) password_confirm: String,
}

// Error
#[derive(Default)]
pub(crate) struct FormError
{
    pub(crate) email: Option<String>,
    pub(crate) email_confirm: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) password_confirm: Option<String>,
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

    // Checks if the email address matches the confirmation email address
    if input.email != input.email_confirm
    {
        let error = t!("pages.signup.form.email_confirm.error.not_match");
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                email_confirm: Some(error),
                ..Default::default()
            },
        };
        return Err(Html(template.render().unwrap()));
    }

    // Checks if the password matches the confirmation password
    if input.password != input.password_confirm
    {
        let error = t!("pages.signup.form.password_confirm.error.not_match");
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                password_confirm: Some(error),
                ..Default::default()
            },
        };
        return Err(Html(template.render().unwrap()));
    }

    // Creates a temporary user.
    let tsx = state.hdb.begin().await.unwrap();
    let temporary_user = TemporaryUserActiveModel
    {
        email: ActiveValue::set(input.email.clone()),
        password: ActiveValue::set(input.password.clone()),
        ..Default::default()
    };
    let temporary_user = match temporary_user
        .validate_and_insert(&tsx)
        .await
    {
        Ok(temporary_user) => temporary_user,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let mut form_error = FormError::default();
            let message = e.get_message();
            match e.get_column()
            {
                Some(TemporaryUserColumn::Email) =>
                {
                    form_error.email = Some(message);
                },
                Some(TemporaryUserColumn::Password) =>
                {
                    form_error.password = Some(message);
                },
                _ =>
                {
                    form_error.other = Some(message);
                },
            }
            let template = PageTemplate
            {
                input: input,
                input_error: form_error,
            };
            return Err(Html(template.render().unwrap()));
        },
    };

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
                },
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Sends a confirmation email.
    let email = Message::builder()
        .from(config.system_email_address.parse().unwrap())
        .to(input.email.clone().parse().unwrap())
        .header(ContentType::TEXT_HTML)
        .subject(t!("messages.email.signup.subject"))
        .body
        (
            t!
            (
                "messages.email.signup.body",
                verify_code = temporary_user_code.code
            )
        )
        .unwrap();

    let mailer = get_mailer(&config);
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
