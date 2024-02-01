//------------------------------------------------------------------------------
//! Auth callback handler.
//------------------------------------------------------------------------------

use crate::AppState;

use meower_app_entity::user_token::ActiveModel as UserTokenActiveModel;
use meower_app_entity::user_token::Column as UserTokenColumn;
use meower_app_entity::user_token::Entity as UserTokenEntity;
use meower_entity_ext::ValidateExt;

use axum::extract::{ Extension, State };
use axum::http::{ header, StatusCode };
use axum::response::{ IntoResponse, Response };
use axum_extra::extract::cookie::Cookie;
use reqwest::Client;
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    EntityTrait,
    ModelTrait,
    QueryFilter,
    TransactionTrait,
};
use serde::{ Deserialize, Serialize };
use time::OffsetDateTime;


//------------------------------------------------------------------------------
/// Tokens.
//------------------------------------------------------------------------------
#[derive(Debug, Deserialize, Default, Serialize)]
pub(crate) struct Tokens
{
    access_token: String,
    refresh_token: String,
}


//------------------------------------------------------------------------------
/// Handles.
//------------------------------------------------------------------------------

// GET
pub(crate) async fn get_handler
(
    Extension(user_token): Extension<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let client = Client::new();
    let config = &state.config;

    let tsx = state.hdb.begin().await.unwrap();
    let user_token = match UserTokenEntity::find()
        .filter(UserTokenColumn::Token.eq(user_token))
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user_token) => user_token,
        None =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let endpoint = format!
    (
        "{}/auth/refresh_token/{}",
        config.auth_api_url,
        user_token.refresh_token,
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
    let tokens: Tokens = serde_json::from_str(&res).unwrap_or_default();
    if tokens.access_token.len() <= 0 || tokens.refresh_token.len() <= 0
    {
        tsx.rollback().await.unwrap();
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    if let Err(_) = user_token.delete(&tsx).await
    {
        tsx.rollback().await.unwrap();
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let access_token = tokens.access_token.clone();
    let user_token = UserTokenActiveModel
    {
        access_token: ActiveValue::Set(tokens.access_token.into()),
        refresh_token: ActiveValue::Set(tokens.refresh_token.into()),
        ..Default::default()
    };
    let user_token = match user_token.validate_and_insert(&tsx).await
    {
        Ok(user_token) => user_token,
        Err(_) =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    tsx.commit().await.unwrap();

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
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, cookie)
        .body(access_token)
        .unwrap();
    Ok(res)
}
