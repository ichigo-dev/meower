//------------------------------------------------------------------------------
//! GraphQL request.
//------------------------------------------------------------------------------

use crate::AppState;

use graphql_client::{ Response, GraphQLQuery };
use reqwest::{ StatusCode, Method };
use rust_i18n::t;


//------------------------------------------------------------------------------
/// GraphQL requests.
//------------------------------------------------------------------------------

// Request
#[allow(dead_code)]
pub async fn request_graphql<Q: GraphQLQuery>
(
    state: &AppState,
    endpoint: &str,
    variables: Q::Variables,
    method: Method,
) -> Result<Q::ResponseData, String>
{
    let client = &state.client;
    let config = &state.config;

    let endpoint = endpoint.trim_start_matches('/');
    let url = format!("{}/{}", config.api_url, endpoint);
    let body = Q::build_query(variables);
    let mut response = match client
        .request(method.clone(), url.clone())
        .bearer_auth(config.access_token.lock().unwrap().clone())
        .json(&body)
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
        let mut access_token = config.access_token.lock().unwrap();
        *access_token = token;

        response = match client
            .request(method, url)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(_) => return Err(t!("common.api.network.error")),
        };
    }

    let graphql_response: Response<Q::ResponseData> = match response
        .json()
        .await
    {
        Ok(graphql_response) => graphql_response,
        Err(_) => return Err(t!("common.api.graphql.error")),
    };
    if let Some(errors) = graphql_response.errors
    {
        return Err(errors[0].message.clone());
    }
    let data = match graphql_response.data
    {
        Some(data) => data,
        None => return Err(t!("common.api.graphql.error")),
    };
    Ok(data)
}

// POST
#[allow(dead_code)]
pub async fn post_graphql<Q: GraphQLQuery>
(
    state: &AppState,
    endpoint: &str,
    variables: Q::Variables,
) -> Result<Q::ResponseData, String>
{
    request_graphql::<Q>(state, endpoint, variables, Method::POST).await
}
