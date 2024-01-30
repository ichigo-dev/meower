//------------------------------------------------------------------------------
//! GraphQL.
//------------------------------------------------------------------------------

mod query;

use query::account::AccountQuery;
use crate::AppState;

use async_graphql::{
    EmptySubscription,
    MergedObject,
    Object,
    Request,
    Response,
    Schema,
};
use axum::extract::{ Extension, State };
use axum::response::Json;


//------------------------------------------------------------------------------
/// GraphQL root.
//------------------------------------------------------------------------------
pub type GraphqlRoot = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn graphql_handler
(
    state: State<AppState>,
    schema: Extension<GraphqlRoot>,
    req: Json<Request>,
) -> Json<Response>
{
    schema.execute(req.0.data(state)).await.into()
}


//------------------------------------------------------------------------------
/// Query root.
//------------------------------------------------------------------------------
#[derive(MergedObject)]
pub struct QueryRoot(AccountQuery);


//------------------------------------------------------------------------------
/// Mutation root.
//------------------------------------------------------------------------------
pub struct MutationRoot;

#[Object]
impl MutationRoot
{
    async fn hello(&self) -> String
    {
        "Hello, world!".to_string()
    }
}
