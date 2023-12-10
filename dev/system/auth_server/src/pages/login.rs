//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, I18n };
use meower_entity::user::Model as UserModel;
use meower_entity::temporary_user::Model as TemporaryUserModel;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::TransactionTrait;


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
#[derive(Template, Default)]
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
    Extension(i18n): Extension<I18n>,
) -> impl IntoResponse
{
    let template = LoginTemplate
    {
        i18n,
        ..Default::default()
    };
    Html(template.render().unwrap())
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

    let tsx = hdb.begin().await.unwrap();
    if let Err(e) = try_login(&tsx, &input, &i18n).await
    {
        tsx.rollback().await.unwrap();
        let template = LoginTemplate
        {
            i18n,
            input,
            errors: vec![e],
            ..Default::default()
        };
        return Err(Html(template.render().unwrap()));
    }

    // Proxies to the frontend.
    tsx.commit().await.unwrap();
    let cookie = Auth::make_jwt_cookie(&config);
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(Body::empty())
        .unwrap();
    Ok(response)
}


//------------------------------------------------------------------------------
/// Tries to login.
//------------------------------------------------------------------------------
async fn try_login<C>
(
    hdb: &C,
    input: &LoginForm,
    i18n: &I18n,
) -> Result<(), String>
where
    C: ConnectionTrait,
{
    // Try to login.
    if let Some(user) = UserModel::find_by_email(hdb, &input.email).await
    {
        if user.try_login(hdb, &input.password).await == false
        {
            return Err
            (
                i18n.get("auth_server.login.form.error.invalid_password")
            );
        }
    }
    else
    {
        if TemporaryUserModel::find_by_email(hdb, &input.email).await.is_some()
        {
            return Err
            (
                i18n.get("auth_server.login.form.error.user_not_verified")
            );
        }

        return Err(i18n.get("auth_server.login.form.error.user_not_found"));
    }

    Ok(())
}
