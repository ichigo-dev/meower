//------------------------------------------------------------------------------
//! GraphQL request.
//------------------------------------------------------------------------------

use crate::AppState;

use reqwest::{ StatusCode, Method };
use graphql_client::{ Response, GraphQLQuery };
use thiserror::Error;


//------------------------------------------------------------------------------
/// GraphQL requests.
//------------------------------------------------------------------------------

// Error
#[derive(Debug, Error)]
pub enum Error
{
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),

    #[error("Refresh token error")]
    RefreshToken,
}

// Request
#[allow(dead_code)]
pub async fn request_graphql<Q: GraphQLQuery>
(
    state: &AppState,
    endpoint: &str,
    variables: Q::Variables,
    method: Method,
) -> Result<Response<Q::ResponseData>, Error>
{
    let client = &state.client;
    let config = &state.config;

    let endpoint = endpoint.trim_start_matches('/');
    let url = format!("{}/{}", config.api_url, endpoint);
    let body = Q::build_query(variables);
    let response = client
        .request(method.clone(), url.clone())
        .bearer_auth(config.access_token.lock().unwrap().clone())
        .json(&body)
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

        let graphql_response = client
            .request(method, url)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        return Ok(graphql_response);
    }

    let graphql_response = response.json().await?;
    Ok(graphql_response)
}

// POST
#[allow(dead_code)]
pub async fn post_graphql<Q: GraphQLQuery>
(
    state: &AppState,
    endpoint: &str,
    variables: Q::Variables,
) -> Result<Response<Q::ResponseData>, Error>
{
    request_graphql::<Q>(state, endpoint, variables, Method::POST).await
}
