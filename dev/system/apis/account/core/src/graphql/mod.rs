//------------------------------------------------------------------------------
//! GraphQL.
//------------------------------------------------------------------------------

pub(crate) mod query;

use crate::state::AppState;
use query::account::AccountQuery;

use async_graphql::{
    EmptySubscription,
    MergedObject,
    Object,
    Request,
    Response,
    Schema,
};
use axum::extract::{ Extension, Json, State };

type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;


//------------------------------------------------------------------------------
/// GraphQL handler
//------------------------------------------------------------------------------
pub(crate) async fn handler
(
    state: State<AppState>,
    schema: Extension<AppSchema>,
    req: Json<Request>,
) -> Json<Response>
{
    schema.execute(req.0.data(state)).await.into()
}


//------------------------------------------------------------------------------
/// Query root.
//------------------------------------------------------------------------------
#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(AccountQuery);


//------------------------------------------------------------------------------
/// Mutation root.
//------------------------------------------------------------------------------
pub(crate) struct MutationRoot;

#[Object]
impl MutationRoot
{
    async fn hello( &self ) -> String
    {
        "Hello, world!".to_string()
    }
}
