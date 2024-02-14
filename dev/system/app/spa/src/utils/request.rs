//------------------------------------------------------------------------------
//! API request.
//------------------------------------------------------------------------------

use crate::AppState;

use reqwest::{ StatusCode, Method, Response };
use rust_i18n::t;


//------------------------------------------------------------------------------
/// Refreshes the access token.
//------------------------------------------------------------------------------
pub async fn refresh_token( state: &mut AppState ) -> Result<(), String>
{
    let client = &state.client;
    let config = &mut state.config;

    let refresh_url = format!("{}/auth/refresh_token", config.app_url);
    let refresh_response = match client.get(refresh_url).send().await
    {
        Ok(refresh_response) => refresh_response,
        Err(_) => return Err(t!("common.api.network.error")),
    };
    let token = refresh_response.text().await.unwrap_or_default();
    if token.is_empty()
    {
        return Err(t!("common.api.unauthorized.error"));
    }
    config.update_access_token(&token);
    Ok(())
}


//------------------------------------------------------------------------------
/// API requests.
//------------------------------------------------------------------------------

// Tries a request.
pub async fn try_request
(
    state: &mut AppState,
    endpoint: &str,
    body: &str,
    method: Method,
) -> Result<Response, String>
{
    let client = &state.client;
    let config = &mut state.config;
    let body = body.to_string();

    let endpoint = endpoint.trim_start_matches('/');
    let url = format!("{}/{}", config.api_url, endpoint);
    match client
        .request(method.clone(), url.clone())
        .bearer_auth(&config.access_token)
        .body(body.clone())
        .send()
        .await
    {
        Ok(response) => Ok(response),
        Err(_) => Err(t!("common.api.network.error")),
    }
}

// Request
pub async fn request
(
    state: &mut AppState,
    endpoint: &str,
    body: &str,
    method: Method,
) -> Result<Response, String>
{
    let max_retry_count = 3;
    let mut retry_count = 0;
    while retry_count < max_retry_count
    {
        let response = match try_request
        (
            state,
            endpoint,
            body,
            method.clone()
        ).await
        {
            Ok(response) => response,
            Err(_) => return Err(t!("common.api.network.error")),
        };

        if response.status() == StatusCode::OK
        {
            return Ok(response);
        }

        retry_count += 1;
        let _ = refresh_token(state).await;
    }
    Err(t!("common.api.unauthorized.error"))
}

// GET
#[allow(dead_code)]
pub async fn get
(
    state: &mut AppState,
    endpoint: &str,
    body: &str,
) -> Result<Response, String>
{
    request(state, endpoint, body, Method::GET).await
}

// POST
#[allow(dead_code)]
pub async fn post
(
    state: &mut AppState,
    endpoint: &str,
    body: &str,
) -> Result<Response, String>
{
    request(state, endpoint, body, Method::POST).await
}

// PUT
#[allow(dead_code)]
pub async fn put
(
    state: &mut AppState,
    endpoint: &str,
    body: &str,
) -> Result<Response, String>
{
    request(state, endpoint, body, Method::PUT).await
}

// DELETE
#[allow(dead_code)]
pub async fn delete
(
    state: &mut AppState,
    endpoint: &str,
    body: &str,
) -> Result<Response, String>
{
    request(state, endpoint, body, Method::DELETE).await
}
