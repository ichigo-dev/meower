//------------------------------------------------------------------------------
//! APIs.
//------------------------------------------------------------------------------

pub mod graphql;

use crate::AppState;

use reqwest::{ StatusCode, Method };


//------------------------------------------------------------------------------
/// API requests.
//------------------------------------------------------------------------------

// Request
#[allow(dead_code)]
pub(crate) async fn request
(
    state: &AppState,
    endpoint: &str,
    body: &str,
    method: Method,
) -> Result<String, ()>
{
    let client = &state.client;
    let config = &state.config;

    let endpoint = endpoint.trim_start_matches('/');
    let url = format!("{}/{}", config.api_url, endpoint);
    let response = match client
        .request(method.clone(), url.clone())
        .bearer_auth(config.access_token.lock().unwrap().clone())
        .json(&body)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(()),
    };

    // If the authentication status is invalid, refresh the token and try again.
    if response.status() == StatusCode::UNAUTHORIZED
    {
        let refresh_url = format!("{}/auth/refresh_token", config.app_url);
        let response = match client.get(refresh_url).send().await
        {
            Ok(response) => response,
            Err(_) => return Err(()),
        };

        let token = response.text().await.unwrap_or_default();
        if token.is_empty()
        {
            return Err(());
        }

        let mut access_token = config.access_token.lock().unwrap();
        *access_token = token;

        match client
            .request(method, url)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await
        {
            Ok(response) =>
            {
                return Ok(response.text().await.unwrap_or_default());
            },
            Err(_) => return Err(()),
        };
    }

    Ok(response.text().await.unwrap_or_default())
}

// GET
#[allow(dead_code)]
pub(crate) async fn get
(
    state: &AppState,
    endpoint: &str,
    body: &str,
) -> Result<String, ()>
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
) -> Result<String, ()>
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
) -> Result<String, ()>
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
) -> Result<String, ()>
{
    request(state, endpoint, body, Method::DELETE).await
}
