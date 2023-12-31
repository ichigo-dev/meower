//------------------------------------------------------------------------------
//! Forgot password page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::utils::email::get_mailer;
use crate::pages::forgot_password_success::PageTemplate as PasswordForgotSuccessPageTemplate;
use meower_entity::traits::validate::ValidateExt;
use meower_entity::user::Entity as UserEntity;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::reset_password_token::ActiveModel as ActiveResetPasswordToken;

use askama::Template;
use axum::response::{ Html, IntoResponse };
use axum::extract::{ State, Form };
use lettre::AsyncTransport;
use lettre::Message;
use lettre::message::header::ContentType;
use rust_i18n::t;
use serde::Deserialize;
use sea_orm::{ ActiveValue, TransactionTrait };


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "forgot_password.html")]
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
}

// Error
#[derive(Default)]
pub(crate) struct FormError
{
    pub(crate) email: Option<String>,
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

    // Finds user.
    let user = match UserEntity::find_by_email(&input.email)
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            match TemporaryUserEntity::find_by_email(&input.email)
                    .one(&tsx)
                    .await
                    .unwrap()
            {
                Some(_) =>
                {
                    tsx.rollback().await.unwrap();
                    let error = t!("pages.forgot_password.form.email.error.not_verified");
                    let template = PageTemplate
                    {
                        input: input,
                        input_error: FormError
                        {
                            other: Some(error),
                            ..Default::default()
                        },
                    };
                    return Err(Html(template.render().unwrap()));
                },
                None =>
                {
                    // Ends normally as a countermeasure against blind SQL
                    // injection.
                    tsx.rollback().await.unwrap();
                    let template = PasswordForgotSuccessPageTemplate::default();
                    return Ok(Html(template.render().unwrap()));
                },
            };
        },
    };

    // Creates a reset password token.
    let reset_password_token = ActiveResetPasswordToken
    {
        user_id: ActiveValue::Set(user.user_id),
        ..Default::default()
    };
    let reset_password_token = match reset_password_token
        .validate_and_insert(&tsx)
        .await
    {
        Ok(reset_password_token) => reset_password_token,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = PageTemplate
            {
                input: input,
                input_error: FormError
                {
                    other: Some(e.get_error_message().1),
                    ..Default::default()
                },
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Sends a confirmation email.
    let mailer = get_mailer(&config);
    let reset_password_url = format!
    (
        "{}/auth/reset_password/{}",
        config.system_url,
        reset_password_token.token,
    );
    let email = Message::builder()
        .from(config.system_email_address.parse().unwrap())
        .to(input.email.clone().parse().unwrap())
        .header(ContentType::TEXT_HTML)
        .subject(t!("messages.email.reset_password.subject"))
        .body
        (
            t!
            (
                "messages.email.reset_password.body",
                reset_password_url = reset_password_url
            )
        )
        .unwrap();
    match mailer.send(email).await
    {
        Ok(_) => {},
        Err(e) =>
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
        },
    }

    tsx.commit().await.unwrap();
    let template = PasswordForgotSuccessPageTemplate::default();
    return Ok(Html(template.render().unwrap()));
}
