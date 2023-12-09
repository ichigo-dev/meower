//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use meower_entity::user::Model as UserModel;
use crate::{ AppState, Auth, I18n };

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Redirect, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------
#[derive(Deserialize, Debug, Default)]
pub(crate) struct LoginForm
{
    email: String,
    password: String,
}


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate
{
    pub(crate) i18n: I18n,
    pub(crate) input: LoginForm,
    pub(crate) errors: Vec<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    Extension(auth): Extension<Auth>,
    Extension(i18n): Extension<I18n>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    if auth.is_logined().await
    {
        return Err(Redirect::to("/"));
    }

    let template = LoginTemplate
    {
        i18n,
        input: LoginForm::default(),
        errors: Vec::new(),
    };
    Ok(Html(template.render().unwrap()))
}

// POST
pub(crate) async fn post_handler
(
    State(state): State<AppState>,
    Extension(i18n): Extension<I18n>,
    Form(input): Form<LoginForm>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let hdb = state.hdb();
    let config = state.config();

    // Try to login.
    if let Some(user) = UserModel::find_by_email(hdb, &input.email).await
    {
        if user.try_login(&hdb, &input.password).await == false
        {
            let errors = vec!
            [
                i18n.get("auth_server.login.form.error.invalid_password")
            ];
            let template = LoginTemplate { i18n, input, errors };
            return Err(Html(template.render().unwrap()));
        }
    }
    else
    {
        let errors = vec!
        [
            i18n.get("auth_server.login.form.error.user_not_found")
        ];
        let template = LoginTemplate { i18n, input, errors };
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
