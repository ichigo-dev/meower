//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, JWT_COOKIE_KEY, Validator };

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Redirect, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use axum_extra::extract::cookie::{ Cookie, SameSite };
use time::Duration;
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Signup page template.
//------------------------------------------------------------------------------
#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate
{
    errors: Vec<String>,
}

impl Default for SignupTemplate
{
    fn default() -> Self
    {
        Self { errors: Vec::new() }
    }
}


//------------------------------------------------------------------------------
/// Handles signup page.
//------------------------------------------------------------------------------
pub(crate) async fn get_handler
(
    Extension(auth): Extension<Auth>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    if auth.is_logined().await
    {
        return Err(Redirect::to("/"));
    }

    let template = SignupTemplate::default();
    Ok(Html(template.render().unwrap()))
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug)]
pub(crate) struct SignupForm
{
    email: String,
    email_confirm: String,
    password: String,
    password_confirm: String,
}


//------------------------------------------------------------------------------
/// Handler for signup form.
//------------------------------------------------------------------------------
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(auth): Extension<Auth>,
    Form(input): Form<SignupForm>,
) -> impl IntoResponse
{
    // Validates email.
    let mut email_validator = Validator::new(&input.email)
        .not_empty("Email is empty.")
        .max_len(255, "Email is too long.")
        .is_email("Email is invalid.")
        .same(&input.email_confirm, "Emails do not match.")
        .validate();
    if email_validator.validate() == false
    {
        let errors = email_validator.errors();
        let template = SignupTemplate { errors: errors.to_vec() };
        return Html(template.render().unwrap());
    }

    // Validates password.
    let mut password_validator = Validator::new(&input.password)
        .not_empty("Password is empty.")
        .min_len(8, "Password is too short.")
        .max_len(255, "Password is too long.")
        .same(&input.password_confirm, "Passwords do not match.")
        .validate();
    if password_validator.validate() == false
    {
        let errors = password_validator.errors();
        let template = SignupTemplate { errors: errors.to_vec() };
        return Html(template.render().unwrap());
    }

    let template = SignupTemplate::default();
    return Html(template.render().unwrap());
}
