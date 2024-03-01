//------------------------------------------------------------------------------
//! GraphQL request.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::utils::request::refresh_token;

use graphql_client::{ Response, GraphQLQuery, QueryBody };
use reqwest::{ Method, Response as ReqwestResponse, StatusCode };
use rust_i18n::t;


//------------------------------------------------------------------------------
/// GraphQL requests.
//------------------------------------------------------------------------------

// Tries request
#[allow(dead_code)]
pub async fn try_request_graphql_inner<Q: GraphQLQuery>
(
    state: &mut AppState,
    endpoint: &str,
    body: &QueryBody<Q::Variables>,
    method: Method,
) -> Result<ReqwestResponse, String>
{
    let client = &state.client;
    let config = &mut state.config;

    let endpoint = endpoint.trim_start_matches('/');
    let url = format!("{}/{}", config.api_url, endpoint);
    match client
        .request(method.clone(), url.clone())
        .bearer_auth(&config.access_token)
        .json(&body)
        .send()
        .await
    {
        Ok(response) => Ok(response),
        Err(_) => Err(t!("common.api.network.error")),
    }
}

// Request
pub async fn request_graphql_inner<Q: GraphQLQuery>
(
    state: &mut AppState,
    endpoint: &str,
    variables: Q::Variables,
    method: Method,
) -> Result<ReqwestResponse, String>
{
    let max_retry_count = 3;
    let mut retry_count = 0;
    let body = Q::build_query(variables);
    while retry_count < max_retry_count
    {
        let response = match try_request_graphql_inner::<Q>
        (
            state,
            endpoint,
            &body,
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

pub async fn request_graphql<Q: GraphQLQuery>
(
    state: &mut AppState,
    endpoint: &str,
    variables: Q::Variables,
    method: Method,
) -> Result<Q::ResponseData, String>
{
    let response = match request_graphql_inner::<Q>
    (
        state,
        endpoint,
        variables,
        method.clone()
    ).await
    {
        Ok(response) => response,
        Err(_) => return Err(t!("common.api.network.error")),
    };

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
    state: &mut AppState,
    endpoint: &str,
    variables: Q::Variables,
) -> Result<Q::ResponseData, String>
{
    request_graphql::<Q>
    (
        state,
        endpoint,
        variables,
        Method::POST,
    ).await
}
