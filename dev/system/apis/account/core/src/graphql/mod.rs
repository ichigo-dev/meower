//------------------------------------------------------------------------------
//! GraphQL.
//------------------------------------------------------------------------------

mod query;
mod mutation;

pub(crate) use query::QueryRoot;
pub(crate) use mutation::MutationRoot;

use crate::state::AppState;
use meower_shared::JwtClaims;

use std::str::from_utf8;
use std::sync::Arc;

use async_graphql::{ Request, Response };
use axum::extract::{ Json, State };
use axum::http::{ header, HeaderMap };
use base64::prelude::*;
use sea_orm::TransactionTrait;


//------------------------------------------------------------------------------
/// GraphQL handler
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    state: State<AppState>,
    headers: HeaderMap,
    req: Json<Request>,
) -> Json<Response>
{
    // Extracts the JWT claims from the request.
    let bearer = headers
        .get(header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "");
    let decoded_bytes = &BASE64_STANDARD.decode(bearer).unwrap();
    let auth = from_utf8(decoded_bytes).unwrap();
    let jwt_claims = serde_json::from_str::<JwtClaims>(auth).unwrap();

    // Begins a transaction.
    let tsx = Arc::new(state.hdb.begin().await.unwrap());

    // Executes the query.
    let query = req.0
        .data(state.config.clone())
        .data(state.storage.clone())
        .data(state.enforcer.clone())
        .data(tsx.clone())
        .data(jwt_claims);
    let response = state.schema.execute(query).await;

    // Ends the transaction after all queries are executed.
    let tsx = Arc::try_unwrap(tsx).unwrap();
    if response.is_ok()
    {
        tsx.commit().await.unwrap();
    }
    else
    {
        tsx.rollback().await.unwrap();
    }
    response.into()
}
