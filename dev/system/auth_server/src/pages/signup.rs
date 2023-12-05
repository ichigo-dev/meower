//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use meower_entity::user::ActiveModel as ActiveUser;
use crate::{ AppState, Auth };

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Redirect, IntoResponse };
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::TransactionTrait;


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
    account_name: String,
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
    Form(input): Form<SignupForm>,
) -> impl IntoResponse
{
    let hdb = state.hdb();

    // Checks if the email and password confirmations match.
    if input.email != input.email_confirm
    {
        let template = SignupTemplate { errors: vec!["Emails do not match.".to_string()] };
        return Html(template.render().unwrap());
    }
    if input.password != input.password_confirm
    {
        let template = SignupTemplate { errors: vec!["Passwords do not match.".to_string()] };
        return Html(template.render().unwrap());
    }

    // Creates a new user and account.
    let tsx = hdb.begin().await.unwrap();
    tsx.commit().await.unwrap();

    let template = SignupTemplate::default();
    return Html(template.render().unwrap());
}
