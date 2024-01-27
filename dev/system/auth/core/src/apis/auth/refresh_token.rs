//------------------------------------------------------------------------------
//! Request token handler. 
//------------------------------------------------------------------------------

use crate::AppState;

use meower_auth_entity::client_application::Model as ClientApplicationModel;
use meower_auth_entity::jwt_refresh_token::ActiveModel as JwtRefreshTokenActiveModel;
use meower_auth_entity::jwt_refresh_token::Column as JwtRefreshTokenColumn;
use meower_auth_entity::jwt_refresh_token::Entity as JwtRefreshTokenEntity;
use meower_auth_entity::user::Entity as UserEntity;
use meower_auth_shared::JwtClaims;
use meower_entity_ext::ValidateExt;

use std::fs::File;
use std::io::Read;

use axum::extract::{ Extension, Path, State };
use axum::http::header::HeaderMap;
use axum::http::StatusCode;
use axum::response::{ IntoResponse, Json };
use chrono::{ Utc, Duration };
use jsonwebtoken::{
    encode,
    Header,
    Algorithm,
    EncodingKey,
};
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    EntityTrait,
    ModelTrait,
    QueryFilter,
    TransactionTrait,
};

const JWT_EXPIRATION_MINUTES: i64 = 10;


//------------------------------------------------------------------------------
/// Response.
//------------------------------------------------------------------------------
#[derive(serde::Serialize)]
pub(crate) struct Response
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
    Extension(client_application): Extension<ClientApplicationModel>,
    Path(refresh_token): Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, impl IntoResponse>
{
    let config = &state.config;

    let client_secret = headers
        .get(&config.client_secret_key)
        .map(|value| value.to_str().unwrap().to_string())
        .unwrap_or(String::new());
    if client_application.client_secret != client_secret
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let tsx = state.hdb.begin().await.unwrap();
    let jwt_refresh_token = match JwtRefreshTokenEntity::find()
        .filter(JwtRefreshTokenColumn::Token.eq(refresh_token))
        .one(&state.hdb)
        .await
        .unwrap()
    {
        Some(jwt_refresh_token) => jwt_refresh_token,
        None =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::UNAUTHORIZED);
        },
    };

    let user_id = jwt_refresh_token.user_id;
    if jwt_refresh_token.verify() == false
    {
        jwt_refresh_token.delete(&tsx).await.unwrap();
        tsx.rollback().await.unwrap();
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_refresh_token = JwtRefreshTokenActiveModel
    {
        user_id: ActiveValue::set(user_id),
        ..Default::default()
    };
    let jwt_refresh_token = match jwt_refresh_token
        .validate_and_insert(&tsx)
        .await
    {
        Ok(jwt_refresh_token) => jwt_refresh_token,
        Err(_) =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let user = match UserEntity::find_by_id(jwt_refresh_token.user_id)
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(user) => user,
        None =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    tsx.commit().await.unwrap();

    // Creates JWT claims.
    let now = Utc::now();
    let iat = now.timestamp();
    let duration = Duration::minutes(JWT_EXPIRATION_MINUTES);
    let exp = (now + duration).timestamp();
    let jwt_claims = JwtClaims
    {
        jti: meower_utility::get_random_str(64),
        iss: config.url.clone(),
        sub: user.jwt_subject.clone(),
        aud: client_application.client_id.clone(),
        iat,
        exp,
        nbf: iat,
    };

    // Encodes JWT claims.
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::RS256;

    let key_path = "./env/".to_string() + &config.jwt_private_key;
    let mut key_data = String::new();
    let mut file = File::open(&key_path).unwrap();
    file.read_to_string(&mut key_data).unwrap();

    let key = EncodingKey::from_rsa_pem(key_data.as_bytes()).unwrap();
    let access_token = encode(&header, &jwt_claims, &key).unwrap();
    let res = Response
    {
        access_token: access_token,
        refresh_token: jwt_refresh_token.token,
    };

    Ok(Json(res))
}
