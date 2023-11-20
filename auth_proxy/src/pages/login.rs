//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::AppState;

use askama::Template;
use axum::response::{ Redirect, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use axum_extra::extract::cookie::{ Cookie, SameSite };
use time::Duration;
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Login page template.
//------------------------------------------------------------------------------
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}


//------------------------------------------------------------------------------
/// Handles login page.
//------------------------------------------------------------------------------
pub(crate) async fn get_handler() -> impl IntoResponse
{
    let template = LoginTemplate {};
    let body = template.render().unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap()
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug)]
pub(crate) struct LoginForm
{
    email: String,
    password: String,
}


//------------------------------------------------------------------------------
/// Handler for login form.
//------------------------------------------------------------------------------
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Form(input): Form<LoginForm>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let client = state.client();
    let auth = state.auth();
    let config = state.config();

    // Try to login.
    if auth.login(&input.email, &input.password).await == false
    {
        return Err(Redirect::to("/login"));
    }

    // Makes JWT token.
    let jwt = auth.make_jwt(&config);
    let cookie = Cookie::build("jwt_token", jwt.to_owned())
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(Duration::seconds(config.jwt_expires()))
        .secure(config.debug_mode() == false)
        .finish();

    // Proxies to the frontend.
    let uri = config.proxy_url().parse().unwrap();
    let mut response = client.get(uri).await.unwrap();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}
