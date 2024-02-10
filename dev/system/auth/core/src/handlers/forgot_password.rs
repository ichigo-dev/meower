//------------------------------------------------------------------------------
//! Forgot password page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::utils::email::get_mailer;
use crate::handlers::forgot_password_success::PageTemplate as PasswordForgotSuccessPageTemplate;

use meower_auth_entity::user::Column as UserColumn;
use meower_auth_entity::user::Entity as UserEntity;
use meower_auth_entity::temporary_user::Column as TemporaryUserColumn;
use meower_auth_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_auth_entity::reset_password_token::ActiveModel as ResetPasswordTokenActiveModel;
use meower_entity_ext::ValidateExt;

use askama::Template;
use axum::extract::{ Form, State };
use axum::response::{ Html, IntoResponse };
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
    let user = match UserEntity::find()
        .filter(UserColumn::Email.eq(input.email.clone()))
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            match TemporaryUserEntity::find()
                .filter(TemporaryUserColumn::Email.eq(input.email.clone()))
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
    let reset_password_token = ResetPasswordTokenActiveModel
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
                    other: Some(e.get_message()),
                    ..Default::default()
                },
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Sends a confirmation email.
    let reset_password_url = format!
    (
        "{}/auth/reset_password/{}",
        config.url,
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
    let template = PasswordForgotSuccessPageTemplate::default();
    return Ok(Html(template.render().unwrap()));
}
