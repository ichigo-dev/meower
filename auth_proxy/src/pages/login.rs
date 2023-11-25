//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use meower_entity::user::{ Entity as User };
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
/// Login page template.
//------------------------------------------------------------------------------
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate
{
    errors: Vec<String>,
}

impl Default for LoginTemplate
{
    fn default() -> Self
    {
        Self { errors: Vec::new() }
    }
}


//------------------------------------------------------------------------------
/// Handles login page.
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

    let template = LoginTemplate::default();
    Ok(Html(template.render().unwrap()))
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
    let hdb = state.hdb();
    let config = state.config();

    // Try to login.
    if User::login(hdb, &input.email, &input.password).await == false
    {
        let errors = vec!["Invalid email or password.".to_string()];
        let template = LoginTemplate { errors };
        return Err(Html(template.render().unwrap()));
    }

    // Makes JWT token.
    let jwt = Auth::make_jwt(&config);
    let cookie = Cookie::build(JWT_COOKIE_KEY, jwt.to_owned())
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(Duration::seconds(config.jwt_expires()))
        .secure(config.debug_mode() == false)
        .finish();

    // Proxies to the frontend.
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}
