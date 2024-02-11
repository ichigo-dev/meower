//------------------------------------------------------------------------------
//! GraphQL mutation.
//------------------------------------------------------------------------------

pub mod account;
pub mod account_profile;

use account::AccountMutation;
use account_profile::AccountProfileMutation;

use async_graphql::MergedObject;


//------------------------------------------------------------------------------
/// Mutation root.
//------------------------------------------------------------------------------
#[derive(MergedObject, Default)]
pub(crate) struct MutationRoot
(
    AccountMutation,
    AccountProfileMutation,
);
