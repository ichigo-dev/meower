//------------------------------------------------------------------------------
//! GraphQL query.
//------------------------------------------------------------------------------

mod account;
mod account_profile;
mod group;

use account::AccountQuery;
use account_profile::AccountProfileQuery;
use group::GroupQuery;

use async_graphql::MergedObject;


//------------------------------------------------------------------------------
/// Query root.
//------------------------------------------------------------------------------
#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot
(
    AccountQuery,
    AccountProfileQuery,
    GroupQuery,
);
