//------------------------------------------------------------------------------
//! APIs.
//------------------------------------------------------------------------------

use crate::AppState;


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
    let response = match client.get(url).send().await
    {
        Ok(response) => response,
        Err(_) => return String::new(),
    };
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
