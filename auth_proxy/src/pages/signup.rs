//------------------------------------------------------------------------------
//! Signup page.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, JWT_COOKIE_KEY };

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
}

impl Default for SignupTemplate
{
    fn default() -> Self
    {
        Self {}
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
    password: String,
}


//------------------------------------------------------------------------------
/// Handler for signup form.
//------------------------------------------------------------------------------
pub(crate) async fn post_handler
(
    State(_state): State<AppState>,
    Form(_input): Form<SignupForm>,
) -> impl IntoResponse
{
    let template = SignupTemplate::default();
    return Html(template.render().unwrap());
}
