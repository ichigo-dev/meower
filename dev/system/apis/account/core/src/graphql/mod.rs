//------------------------------------------------------------------------------
//! GraphQL.
//------------------------------------------------------------------------------

pub(crate) mod query;
pub(crate) mod mutation;

use crate::state::AppState;
use query::account::AccountQuery;
use mutation::account::AccountMutation;

use meower_shared::JwtClaims;

use std::str::from_utf8;

use async_graphql::{ MergedObject, Request, Response };
use axum::extract::{ Json, State };
use axum::http::{ header, HeaderMap };
use base64::prelude::*;


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
    let bearer = headers
        .get(header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "");
    let decoded_bytes = &BASE64_STANDARD.decode(bearer).unwrap();
    let auth = from_utf8(decoded_bytes).unwrap();
    let jwt_claims = serde_json::from_str::<JwtClaims>(auth).unwrap();

    let query = req.0
        .data(state.config.clone())
        .data(state.hdb.clone())
        .data(jwt_claims);
    state.schema.execute(query).await.into()
}


//------------------------------------------------------------------------------
/// Query root.
//------------------------------------------------------------------------------
#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(AccountQuery);


//------------------------------------------------------------------------------
/// Mutation root.
//------------------------------------------------------------------------------
#[derive(MergedObject, Default)]
pub(crate) struct MutationRoot(AccountMutation);
