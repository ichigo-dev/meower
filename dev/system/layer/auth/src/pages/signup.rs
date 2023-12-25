//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_type::{ JwtClaim, JWT_CLAIM_KEY };
use meower_entity::traits::validate::ValidateExt;
use meower_entity::user::Error as UserError;
use meower_entity::temporary_user::ActiveModel as ActiveTemporaryUser;
use meower_entity::temporary_user::Error as TemporaryUserError;
use meower_entity::temporary_user_code::ActiveModel as ActiveTemporaryUserCode;

use askama::Template;
use axum::response::{ Html, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use axum_extra::extract::cookie::{ Cookie, SameSite };
use chrono::{ Utc, Duration };
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};
use rust_i18n::t;
use serde::Deserialize;
use sea_orm::{ ActiveValue, TransactionTrait };
use time::{ Duration as TimeDuration, OffsetDateTime };


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template, Default)]
#[template(path = "signup.html")]
struct SignupTemplate
{
    input: SignupForm,
    input_error: SignupFormError,
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------

// Form
#[derive(Deserialize, Debug, Default)]
pub(crate) struct SignupForm
{
    user_account_name: String,
    email: String,
    email_confirm: String,
    password: String,
    password_confirm: String,
}

// Error
#[derive(Default)]
pub(crate) struct SignupFormError
{
    user_account_name: Option<String>,
    email: Option<String>,
    email_confirm: Option<String>,
    password: Option<String>,
    password_confirm: Option<String>,
    other: Option<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler() -> impl IntoResponse
{
    let template = SignupTemplate::default();
    Html(template.render().unwrap())
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Form(input): Form<SignupForm>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let tsx = state.hdb.begin().await.unwrap();

    // Checks if the email address matches the confirmation email address
    if input.email != input.email_confirm
    {
        let error = t!("pages.form.email_confirm.error.not_match");
        let mut template = SignupTemplate
        {
            input: input,
            input_error: SignupFormError
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
        let error = t!("pages.form.password_confirm.error.not_match");
        let mut template = SignupTemplate
        {
            input: input,
            input_error: SignupFormError
            {
                password_confirm: Some(error),
                ..Default::default()
            },
        };
        return Err(Html(template.render().unwrap()));
    }

    // Creates a temporary user.
    let temporary_user = ActiveTemporaryUser
    {
        user_account_name: ActiveValue::set(input.user_account_name.clone()),
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
            let mut signup_form_error = SignupFormError::default();
            let mut template = SignupTemplate
            {
                input: input,
                input_error: signup_form_error,
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Creates a temporary user code.
    let temporary_user_code = ActiveTemporaryUserCode
    {
        temporary_user_id: ActiveValue::set(temporary_user.temporary_user_id),
        ..Default::default()
    };
    let temporary_user_code = match temporary_user_code
        .validate_and_insert(&tsx)
        .await
    {
        Ok(temporary_user_code) => temporary_user_code,
        Err(e) => todo!(),
    };

    tsx.commit().await.unwrap();
    let template = SignupTemplate::default();
    Ok(Html(template.render().unwrap()))
}
