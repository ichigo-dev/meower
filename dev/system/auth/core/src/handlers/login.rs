//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::AppState;

use meower_auth_entity::client_application::Model as ClientApplicationModel;
use meower_auth_entity::jwt_token_code::ActiveModel as JwtTokenCodeActiveModel;
use meower_auth_entity::temporary_user::Column as TemporaryUserColumn;
use meower_auth_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_auth_entity::user::Column as UserColumn;
use meower_auth_entity::user::Entity as UserEntity;
use meower_entity_ext::ValidateExt;

use askama::Template;
use axum::body::Body;
use axum::extract::{ Extension, Form, State };
use axum::http::{ header, StatusCode };
use axum::response::{ Html, IntoResponse, Response };
use axum_extra::extract::cookie::Cookie;
use rust_i18n::t;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    EntityTrait,
    QueryFilter,
    TransactionTrait,
};
use serde::Deserialize;
use time::{ OffsetDateTime, Duration };


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[derive(Template, Default)]
#[template(path = "login.html")]
pub(crate) struct PageTemplate
{
    pub(crate) input: FormData,
    pub(crate) input_error: FormError,
}


//------------------------------------------------------------------------------
/// Form data.
//------------------------------------------------------------------------------

// FormData
#[derive(Deserialize, Debug, Default)]
pub(crate) struct FormData
{
    pub(crate) email: String,
    pub(crate) password: String,
}

// Error
#[derive(Default)]
pub(crate) struct FormError
{
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) other: Option<String>,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler() -> impl IntoResponse
{
    let template = PageTemplate::default();
    Html(template.render().unwrap())
}

// POST
pub(crate) async fn post_handler
(
    Extension(client_application): Extension<ClientApplicationModel>,
    State(state): State<AppState>,
    Form(input): Form<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config;
    let tsx = state.hdb.begin().await.unwrap();

    // Finds user.
    let user = match UserEntity::find()
        .filter(UserColumn::Email.eq(&input.email))
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            let error = match TemporaryUserEntity::find()
                .filter(TemporaryUserColumn::Email.eq(&input.email))
                .one(&tsx)
                .await
                .unwrap()
            {
                Some(_) => t!("pages.login.form.email.error.not_verified"),
                None => t!("pages.login.form.error.failed"),
            };

            tsx.rollback().await.unwrap();
            let template = PageTemplate
            {
                input: input,
                input_error: FormError
                {
                    other: Some(error),
                    ..Default::default()
                },
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Tries to login.
    if user.login(&tsx, &input.password).await == false
    {
        tsx.rollback().await.unwrap();
        let template = PageTemplate
        {
            input: input,
            input_error: FormError
            {
                other: Some(t!("pages.login.form.error.failed")),
                ..Default::default()
            },
        };
        return Err(Html(template.render().unwrap()));
    }

    // Creates JWT token code.
    let code = JwtTokenCodeActiveModel
    {
        user_id: ActiveValue::set(user.user_id),
        ..Default::default()
    };
    let user_jwt_token_code = match code
        .validate_and_insert(&tsx)
        .await
    {
        Ok(user_jwt_token_code) => user_jwt_token_code,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let template = PageTemplate
            {
                input: input,
                input_error: FormError
                {
                    other: Some(e.get_message()),
                    ..Default::default()
                },
            };
            return Err(Html(template.render().unwrap()));
        },
    };
    tsx.commit().await.unwrap();

    // Sets JWT token to cookie.
    let redirect_uri = format!
    (
        "{}?code={}",
        client_application.redirect_uri,
        user_jwt_token_code.code,
    );
    let cookie = Cookie::build((&config.client_id_key, ""))
        .path("/")
        .expires(OffsetDateTime::now_utc() - Duration::days(1))
        .to_string();
    let res = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, redirect_uri)
        .header(header::SET_COOKIE, cookie)
        .body(Body::empty())
        .unwrap();
    Ok(res)
}
