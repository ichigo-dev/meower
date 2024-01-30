//------------------------------------------------------------------------------
//! Account API.
//------------------------------------------------------------------------------

use graphql_client::GraphQLQuery;


//------------------------------------------------------------------------------
/// Gets my accounts.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/account/schema.graphql",
    query_path = "graphql/account/get_my_accounts.graphql",
)]
pub struct GetMyAccounts;
