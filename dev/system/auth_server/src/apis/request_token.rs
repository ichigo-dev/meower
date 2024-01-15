//------------------------------------------------------------------------------
//! Request token endpoint.
//------------------------------------------------------------------------------

use crate::AppState;
use meower_type::JwtClaims;
use meower_entity::traits::validate::ValidateExt;
use meower_entity::user::Entity as User;
use meower_entity::user_jwt_subject::ActiveModel as ActiveUserJwtSubject;
use meower_entity::user_jwt_token_code::Entity as UserJwtTokenCodeEntity;
use meower_entity::user_jwt_refresh_token::ActiveModel as ActiveUserJwtRefreshToken;

use axum::response::{ Json, IntoResponse };
use axum::http::StatusCode;
use axum::extract::{ State, Path };
use chrono::{ Utc, Duration };
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};
use sea_orm::{ ActiveValue, TransactionTrait, ModelTrait };
use serde::Serialize;


//------------------------------------------------------------------------------
/// Response.
//------------------------------------------------------------------------------
#[derive(Serialize)]
struct Response
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
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = state.config;
    let tsx = state.hdb.begin().await.unwrap();

    // Gets user jwt token code.
    let user_token_code = match UserJwtTokenCodeEntity::find_by_code(&code)
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user_token_code) => user_token_code,
        None =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::BAD_REQUEST);
        },
    };

    let user = match user_token_code
        .find_related(User)
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::BAD_REQUEST);
        },
    };

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
        Err(_) =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::BAD_REQUEST);
        },
    };

    // Creates JWT claims.
    let now = Utc::now();
    let iat = now.timestamp();
    let duration = Duration::minutes(config.jwt_expiration_minutes);
    let exp = (now + duration).timestamp();
    let user_account_name = match user
        .get_last_logined_user_account(&tsx)
        .await
    {
        Some(user_account) => user_account.user_account_name,
        None => "".to_string(),
    };
    let jwt_claims = JwtClaims
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

    // Encodes JWT claims.
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::HS256;
    let key = EncodingKey::from_secret(&config.jwt_secret.as_bytes());
    let access_token = encode(&header, &jwt_claims, &key).unwrap();

    // Creates refresh token.
    let active_refresh_token = ActiveUserJwtRefreshToken
    {
        ..Default::default()
    };
    let refresh_token = match active_refresh_token
        .validate_and_insert(&tsx)
        .await
    {
        Ok(refresh_token) => refresh_token,
        Err(_) =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::BAD_REQUEST);
        },
    };

    let response = Response
    {
        access_token,
        refresh_token: refresh_token.token,
    };
    tsx.commit().await.unwrap();

    Ok(Json(response))
}
