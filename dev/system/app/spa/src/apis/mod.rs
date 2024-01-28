//------------------------------------------------------------------------------
//! APIs.
//------------------------------------------------------------------------------

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
    method: Method,
) -> String
{
    let client = &state.client;
    let config = &state.config;

    let url = format!("{}/{}", config.api_url, endpoint);
    let response = match client
        .request(method.clone(), url.clone())
        .bearer_auth(config.access_token.lock().unwrap().clone())
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return String::new(),
    };

    // If the authentication status is invalid, refresh the token and try again.
    if response.status() == StatusCode::UNAUTHORIZED
    {
        let refresh_url = format!("{}/auth/refresh_token", config.app_url);
        let response = match client.get(refresh_url).send().await
        {
            Ok(response) => response,
            Err(_) => return String::new(),
        };
        let mut access_token = config.access_token.lock().unwrap();
        *access_token = response.text().await.unwrap_or_default();

        match client
            .request(method, url)
            .bearer_auth(access_token)
            .send()
            .await
        {
            Ok(response) => return response.text().await.unwrap_or_default(),
            Err(_) => return String::new(),
        };
    }

    response.text().await.unwrap_or_default()
}

pub(crate) async fn get( state: &AppState, endpoint: &str ) -> String
{
    request(state, endpoint, Method::GET).await
}

// POST
#[allow(dead_code)]
pub(crate) async fn post( state: &AppState, endpoint: &str ) -> String
{
    request(state, endpoint, Method::POST).await
}

// PUT
#[allow(dead_code)]
pub(crate) async fn put( state: &AppState, endpoint: &str ) -> String
{
    request(state, endpoint, Method::PUT).await
}

// DELETE
#[allow(dead_code)]
pub(crate) async fn delete( state: &AppState, endpoint: &str ) -> String
{
    request(state, endpoint, Method::DELETE).await
}
