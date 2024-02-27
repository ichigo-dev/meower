//------------------------------------------------------------------------------
//! Request token handler. 
//------------------------------------------------------------------------------

use crate::AppState;
use crate::utils::jwt::create_jwt_token;

use meower_auth_entity::client_application::Model as ClientApplicationModel;
use meower_auth_entity::jwt_refresh_token::ActiveModel as JwtRefreshTokenActiveModel;
use meower_auth_entity::jwt_token_code::Column as JwtTokenCodeColumn;
use meower_auth_entity::jwt_token_code::Entity as JwtTokenCodeEntity;
use meower_auth_entity::user::Entity as UserEntity;
use meower_entity_ext::ValidateExt;

use axum::extract::{ Extension, Path, State };
use axum::http::header::HeaderMap;
use axum::http::StatusCode;
use axum::response::{ IntoResponse, Json };
use sea_orm::{
    ActiveValue,
    ColumnTrait,
    EntityTrait,
    ModelTrait,
    QueryFilter,
    TransactionTrait,
};


//------------------------------------------------------------------------------
/// Tokens.
//------------------------------------------------------------------------------
#[derive(serde::Serialize)]
pub(crate) struct Response
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
    Extension(client_application): Extension<ClientApplicationModel>,
    Path(code): Path<String>,
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
    let jwt_token_code = match JwtTokenCodeEntity::find()
        .filter(JwtTokenCodeColumn::Code.eq(code))
        .one(&tsx)
        .await
        .unwrap()
    {
        Some(jwt_token_code) => jwt_token_code,
        None =>
        {
            tsx.rollback().await.unwrap();
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if jwt_token_code.verify() == false
    {
        tsx.rollback().await.unwrap();
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_refresh_token = JwtRefreshTokenActiveModel
    {
        user_id: ActiveValue::set(jwt_token_code.user_id),
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

    if jwt_token_code.delete(&tsx).await.is_err()
    {
        tsx.rollback().await.unwrap();
        return Err(StatusCode::UNAUTHORIZED);
    }

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

    let access_token = create_jwt_token(&config, &user, &client_application);
    let res = Response
    {
        public_user_id: user.public_user_id,
        user_email: user.email,
        access_token: access_token,
        refresh_token: jwt_refresh_token.token,
    };

    Ok(Json(res))
}
