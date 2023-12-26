//------------------------------------------------------------------------------
//! Login page.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_type::{ JwtClaim, JWT_CLAIM_KEY };
use meower_entity::traits::validate::ValidateExt;
use meower_entity::user::Entity as UserEntity;
use meower_entity::temporary_user::Entity as TemporaryUserEntity;
use meower_entity::user_jwt_subject::ActiveModel as ActiveUserJwtSubject;
use meower_entity::user_jwt_subject::Error as UserJwtSubjectError;

use askama::Template;
use axum::response::{ Html, Response, IntoResponse };
use axum::http::{ header, StatusCode };
use axum::body::Body;
use axum::extract::{ State, Form };
use axum_extra::extract::cookie::{ Cookie, SameSite };
use chrono::{ Utc, Duration };
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};
use rust_i18n::t;
use serde::Deserialize;
use sea_orm::{ ActiveValue, TransactionTrait };
use time::{ Duration as TimeDuration, OffsetDateTime };


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

    // Creates JWT subject.
    let active_user_jwt_subject = ActiveUserJwtSubject
    {
        user_id: ActiveValue::set(user.user_id),
        ..Default::default()
    };
    let user_jwt_subject = match active_user_jwt_subject
        .validate_and_insert(&tsx)
        .await
    {
        Ok(subject) => subject,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            let error = match e
            {
                UserJwtSubjectError::DbError(e) => e.to_string(),
            };
            let template = PageTemplate
            {
                input: input,
                input_error: FormError
                {
                    email: Some(error),
                    ..Default::default()
                },
            };
            return Err(Html(template.render().unwrap()));
        },
    };

    // Creates JWT claim.
    let now = Utc::now();
    let iat = now.timestamp();
    let duration = Duration::minutes(config.jwt_expiration_minutes);
    let exp = (now + duration).timestamp();
    let user_account_name = match user.get_last_logined_user_account(&tsx).await
    {
        Some(user_account) => user_account.user_account_name,
        None => "".to_string(),
    };
    let jwt_claim = JwtClaim
    {
        iss: config.jwt_issue.clone(),
        sub: user_jwt_subject.subject.to_string(),
        aud: config.jwt_audience.clone(),
        iat,
        exp,
        nbf: iat,
        jti: "meower".to_string(),
        uan: user_account_name,
    };

    // Encodes JWT claim.
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::HS256;
    let key = EncodingKey::from_secret(&config.jwt_secret.as_bytes());
    let jwt = encode(&header, &jwt_claim, &key).unwrap();

    // Builds JWT claim cookie.
    let offset_date_time = OffsetDateTime::now_utc();
    let time_duration = TimeDuration::minutes(config.jwt_expiration_minutes);
    let jwt_claim_cookie = Cookie::build((JWT_CLAIM_KEY, jwt.to_owned()))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .max_age(time_duration)
        .expires(offset_date_time + time_duration)
        .secure(config.debug_mode == false)
        .to_string();
    tsx.commit().await.unwrap();

    // Sets JWT token to cookie.
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, jwt_claim_cookie)
        .body(Body::empty())
        .unwrap();
    Ok(response)
}
