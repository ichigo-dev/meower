//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::{ AppState, Auth, I18n };
use meower_entity::user::Entity as UserEntity;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::user_jwt_subject::ActiveModel as ActiveUserJwtSubject;

use askama::Template;
use axum::Extension;
use axum::response::{ Html, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use serde::Deserialize;
use sea_orm::prelude::*;
use sea_orm::{ ActiveValue, TransactionTrait };


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
    let subject = match try_login(&tsx, &input, &i18n).await
    {
        Ok(user) => user,
        Err(e) =>
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
    };

    // Proxies to the frontend.
    tsx.commit().await.unwrap();
    let cookie = Auth::make_jwt_cookie(&config, &subject);
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
) -> Result<String, String>
where
    C: ConnectionTrait,
{
    // Try to login.
    let user = match UserEntity::find_by_email(hdb, &input.email).await
    {
        Some(user) => user,
        None =>
        {
            let error = match TemporaryUserEntity::find_by_email
            (
                hdb,
                &input.email,
            ).await
            {
                Some(_) =>
                {
                    i18n.get("auth_server.login.form.error.user_not_verified")
                },
                None =>
                {
                    i18n.get("auth_server.login.form.error.user_not_found")
                },
            };
            return Err(error);
        }
    };

    if user.try_login(hdb, &input.password).await == false
    {
        return Err
        (
            i18n.get("auth_server.login.form.error.invalid_password")
        );
    }

    // Creates a JWT subject.
    let user_jwt_subject = ActiveUserJwtSubject
    {
        user_id: ActiveValue::Set(user.user_id),
        ..Default::default()
    };
    let user_jwt_subject = match user_jwt_subject.insert(hdb).await
    {
        Ok(subject) => subject,
        Err(e) => return Err(e.to_string()),
    };

    let subject = user_jwt_subject.subject;
    Ok(subject)
}
