//------------------------------------------------------------------------------
//! GraphQL.
//------------------------------------------------------------------------------

mod query;

use query::account::AccountQuery;

use async_graphql::{ MergedObject, Object };


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
    async fn hello(&self) -> String
    {
        "Hello, world!".to_string()
    }
}
