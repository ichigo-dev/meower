//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_entity::traits::validate::ValidateExt;
use meower_entity::user::Entity as UserEntity;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::user_jwt_token_code::ActiveModel as UserJwtTokenCodeActiveModel;
use meower_entity::user_jwt_token_code::Error as UserJwtTokenCodeError;

use askama::Template;
use axum::response::{ Html, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use rust_i18n::t;
use serde::Deserialize;
use sea_orm::{ ActiveValue, TransactionTrait };


//------------------------------------------------------------------------------
/// Page template.
//------------------------------------------------------------------------------
#[allow(dead_code)]
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
    State(state): State<AppState>,
    Form(input): Form<FormData>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config;
    let tsx = state.hdb.begin().await.unwrap();

    // Finds user.
    let user = match UserEntity::find_by_email(&input.email)
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            let error = match TemporaryUserEntity::find_by_email(&input.email)
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
    if user.try_login(&tsx, &input.password).await == false
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
    let code = UserJwtTokenCodeActiveModel
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
            let error = match e
            {
                UserJwtTokenCodeError::DbError(e) => e.to_string(),
            };
            let template = PageTemplate
            {
                input: input,
                input_error: FormError
                {
                    other: Some(error.clone()),
                    ..Default::default()
                },
            };
            return Err(Html(template.render().unwrap()));
        },
    };
    tsx.commit().await.unwrap();

    // Sets JWT token to cookie.
    let url = format!
    (
        "{}/login_callback?code={}",
        config.spa_url.clone().trim_end_matches('/'),
        user_jwt_token_code.code,
    );
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, url)
        .body(Body::empty())
        .unwrap();
    Ok(response)
}
