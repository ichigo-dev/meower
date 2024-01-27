//------------------------------------------------------------------------------
//! APIs.
//------------------------------------------------------------------------------

use crate::AppState;

use reqwest::StatusCode;


//------------------------------------------------------------------------------
/// API requests.
//------------------------------------------------------------------------------

// GET
#[allow(dead_code)]
pub(crate) async fn get( state: &AppState, endpoint: &str ) -> String
{
    let client = &state.client;
    let config = &state.config;

    let url = format!("{}/{}", config.api_url, endpoint);
    let response = match client.get(url.clone()).send().await
    {
        Ok(response) => response,
        Err(_) => return String::new(),
    };

    if response.status() == StatusCode::UNAUTHORIZED
    {
        let refresh_url = format!
        (
            "{}/auth/refresh_token",
            config.app_url
        );
        let response = match client.get(refresh_url).send().await
        {
            Ok(response) => response,
            Err(_) => return String::new(),
        };
        let mut access_token = config.access_token.lock().unwrap();
        *access_token = response.text().await.unwrap_or_default();

        let response = match client.get(url).send().await
        {
            Ok(response) =>
            {
                return response.text().await.unwrap_or_default();
            }
            Err(_) => return String::new(),
        };
    }

    response.text().await.unwrap_or_default()
}

// POST
#[allow(dead_code)]
pub(crate) async fn post( state: &AppState, endpoint: &str ) -> String
{
    let client = &state.client;
    let config = &state.config;

    let url = format!("{}/{}", config.api_url, endpoint);
    let response = match client.post(url).send().await
    {
        Ok(response) => response,
        Err(_) => return String::new(),
    };
    response.text().await.unwrap_or_default()
}

// PUT
#[allow(dead_code)]
pub(crate) async fn put( state: &AppState, endpoint: &str ) -> String
{
    let client = &state.client;
    let config = &state.config;

    let url = format!("{}/{}", config.api_url, endpoint);
    let response = match client.put(url).send().await
    {
        Ok(response) => response,
        Err(_) => return String::new(),
    };
    response.text().await.unwrap_or_default()
}

// DELETE
#[allow(dead_code)]
pub(crate) async fn delete( state: &AppState, endpoint: &str ) -> String
{
    let client = &state.client;
    let config = &state.config;

    let url = format!("{}/{}", config.api_url, endpoint);
    let response = match client.delete(url).send().await
    {
        Ok(response) => response,
        Err(_) => return String::new(),
    };
    response.text().await.unwrap_or_default()
}
