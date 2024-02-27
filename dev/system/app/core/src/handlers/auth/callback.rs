//------------------------------------------------------------------------------
//! Auth callback handler.
//------------------------------------------------------------------------------

use crate::AppState;

use meower_app_entity::user_token::ActiveModel as UserTokenActiveModel;
use meower_entity_ext::ValidateExt;

use axum::body::Body;
use axum::extract::{ Query, State };
use axum::http::{ header, StatusCode };
use axum::response::{ IntoResponse, Response };
use axum_extra::extract::cookie::Cookie;
use reqwest::Client;
use sea_orm::ActiveValue;
use serde::Deserialize;
use time::OffsetDateTime;


//------------------------------------------------------------------------------
/// Params.
//------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub(crate) struct Params
{
    pub(crate) code: Option<String>,
}


//------------------------------------------------------------------------------
/// Tokens.
//------------------------------------------------------------------------------
#[derive(Debug, Deserialize, Default)]
pub(crate) struct AuthResponse
{
    public_user_id: String,
    user_email: String,
    access_token: String,
    refresh_token: String,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    State(state): State<AppState>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let code = match params.code
    {
        Some(code) => code,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let client = Client::new();
    let config = &state.config;
    let endpoint = format!
    (
        "{}/auth/request_token/{}",
        config.auth_api_url,
        code,
    );
    let res = client
        .get(&endpoint)
        .header(&config.client_id_key, &config.client_id)
        .header(&config.client_secret_key, &config.client_secret)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let res: AuthResponse = serde_json::from_str(&res).unwrap_or_default();
    if res.access_token.len() <= 0 || res.refresh_token.len() <= 0
    {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let user_token = UserTokenActiveModel
    {
        public_user_id: ActiveValue::Set(res.public_user_id.into()),
        user_email: ActiveValue::Set(res.user_email.into()),
        access_token: ActiveValue::Set(res.access_token.into()),
        refresh_token: ActiveValue::Set(res.refresh_token.into()),
        ..Default::default()
    };
    let user_token = match user_token.validate_and_insert(&state.hdb).await
    {
        Ok(user_token) => user_token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Sets the user token cookie.
    let expire = OffsetDateTime::from_unix_timestamp
    (
        user_token.expired_at.timestamp()
    ).unwrap();
    let cookie = Cookie::build((&config.user_token_key, user_token.token))
        .path("/")
        .secure(true)
        .http_only(true)
        .expires(expire)
        .to_string();
    let res = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie)
        .body(Body::empty())
        .unwrap();
    Ok(res)
}
