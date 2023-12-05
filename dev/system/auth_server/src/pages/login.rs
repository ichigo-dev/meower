//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use meower_entity::user::Model as UserModel;
use crate::{ AppState, Auth };

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Redirect, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
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
    if let Some(user) = UserModel::find_by_email(&hdb, &input.email).await
    {
        if user.try_login(&hdb, &input.password).await == false
        {
            let errors = vec!["Invalid password.".to_string()];
            let template = LoginTemplate { errors };
            return Err(Html(template.render().unwrap()));
        }
    }
    else
    {
        let errors = vec!["Not found the user".to_string()];
        let template = LoginTemplate { errors };
        return Err(Html(template.render().unwrap()));
    }

    // Proxies to the frontend.
    let cookie = Auth::make_jwt_cookie(&config);
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}
