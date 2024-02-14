//------------------------------------------------------------------------------
//! API request.
//------------------------------------------------------------------------------

use crate::AppState;

use reqwest::{ StatusCode, Method, Response };
use rust_i18n::t;


//------------------------------------------------------------------------------
/// API requests.
//------------------------------------------------------------------------------

// Request
#[allow(dead_code)]
pub async fn request
(
    state: &AppState,
    endpoint: &str,
    body: &str,
    method: Method,
) -> Result<Response, String>
{
    let client = &state.client;
    let config = &state.config;
    let body = body.to_string();
    let access_token = config.access_token.read().unwrap();

    let endpoint = endpoint.trim_start_matches('/');
    let url = format!("{}/{}", config.api_url, endpoint);
    let mut response = match client
        .request(method.clone(), url.clone())
        .bearer_auth(&access_token)
        .body(body.clone())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(t!("common.api.network.error")),
    };

    // If the authentication status is invalid, refresh the token and try again.
    if response.status() == StatusCode::UNAUTHORIZED
    {
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
        let mut access_token = config.access_token.write().unwrap();
        *access_token = token;

        response = match client
            .request(method, url)
            .bearer_auth(access_token)
            .body(body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(_) => return Err(t!("common.api.network.error")),
        };
    }

    Ok(response)
}

// GET
#[allow(dead_code)]
pub async fn get
(
    state: &AppState,
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
    state: &AppState,
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
    state: &AppState,
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
    state: &AppState,
    endpoint: &str,
    body: &str,
) -> Result<Response, String>
{
    request(state, endpoint, body, Method::DELETE).await
}
