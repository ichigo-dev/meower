//------------------------------------------------------------------------------
//! Login callback.
//------------------------------------------------------------------------------

use axum::response::IntoResponse;
use axum::extract::{ State, Query };
use reqwest::Client;
use serde::Deserialize;


//------------------------------------------------------------------------------
/// Parameter.
//------------------------------------------------------------------------------
#[derive(Deserialize)]
pub(crate) struct Parameter
{
    code: Option<String>,
}


//------------------------------------------------------------------------------
/// Handle.
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    Query(parameter): Query<Parameter>,
    State(client): State<Client>,
) -> impl IntoResponse
{
    let code = parameter.code.unwrap_or("".to_string());
    let auth_endpoint = "http://balancer:8080/auth/request_token/".to_string() + &code;
    let res = client
        .get(&auth_endpoint)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("res: {}", res);
    res
}
