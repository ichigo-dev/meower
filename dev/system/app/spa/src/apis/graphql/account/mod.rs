//------------------------------------------------------------------------------
//! Account API.
//------------------------------------------------------------------------------

use graphql_client::GraphQLQuery;


//------------------------------------------------------------------------------
/// Gets all accounts.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/account/schema.graphql",
    query_path = "graphql/account/accounts.graphql",
    response_derives = "Debug"
)]
pub struct Accounts;
