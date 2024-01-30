//------------------------------------------------------------------------------
//! APIs.
//------------------------------------------------------------------------------

pub mod graphql;

use crate::AppState;

use reqwest::{ StatusCode, Method };
use thiserror::Error;


//------------------------------------------------------------------------------
/// API requests.
//------------------------------------------------------------------------------

// Error
#[derive(Debug, Error)]
pub(crate) enum Error
{
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),

    #[error("Refresh token error")]
    RefreshToken,
}

// Request
#[allow(dead_code)]
pub(crate) async fn request
(
    state: &AppState,
    endpoint: &str,
    body: &str,
    method: Method,
) -> Result<String, Error>
{
    let client = &state.client;
    let config = &state.config;
    let body = body.to_string();

    let endpoint = endpoint.trim_start_matches('/');
    let url = format!("{}/{}", config.api_url, endpoint);
    let response = client
        .request(method.clone(), url.clone())
        .bearer_auth(config.access_token.lock().unwrap().clone())
        .body(body.clone())
        .send()
        .await?;

    // If the authentication status is invalid, refresh the token and try again.
    if response.status() == StatusCode::UNAUTHORIZED
    {
        let refresh_url = format!("{}/auth/refresh_token", config.app_url);
        let response = client.get(refresh_url).send().await?;
        let token = response.text().await.unwrap_or_default();
        if token.is_empty()
        {
            return Err(Error::RefreshToken);
        }
        let mut access_token = config.access_token.lock().unwrap();
        *access_token = token;

        let response = client
            .request(method, url)
            .bearer_auth(access_token)
            .body(body)
            .send()
            .await?
            .text()
            .await?;
        return Ok(response);
    }

    let response = response.text().await?;
    Ok(response)
}

// GET
#[allow(dead_code)]
pub(crate) async fn get
(
    state: &AppState,
    endpoint: &str,
    body: &str,
) -> Result<String, Error>
{
    request(state, endpoint, body, Method::GET).await
}

// POST
#[allow(dead_code)]
pub(crate) async fn post
(
    state: &AppState,
    endpoint: &str,
    body: &str,
) -> Result<String, Error>
{
    request(state, endpoint, body, Method::POST).await
}

// PUT
#[allow(dead_code)]
pub(crate) async fn put
(
    state: &AppState,
    endpoint: &str,
    body: &str,
) -> Result<String, Error>
{
    request(state, endpoint, body, Method::PUT).await
}

// DELETE
#[allow(dead_code)]
pub(crate) async fn delete
(
    state: &AppState,
    endpoint: &str,
    body: &str,
) -> Result<String, Error>
{
    request(state, endpoint, body, Method::DELETE).await
}
