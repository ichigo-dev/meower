//------------------------------------------------------------------------------
//! Auth callback handler.
//------------------------------------------------------------------------------

use crate::AppState;

use meower_app_entity::user_token::ActiveModel as UserTokenActiveModel;

use axum::extract::{ Query, State };
use axum::http::StatusCode;
use axum::response::IntoResponse;
use reqwest::Client;
use serde::Deserialize;


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
    let endpoint = format!("{}/{}", config.auth_request_token_url, code);
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


    Ok(StatusCode::OK)
}
