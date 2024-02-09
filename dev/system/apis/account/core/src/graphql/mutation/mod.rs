//------------------------------------------------------------------------------
//! GraphQL mutation.
//------------------------------------------------------------------------------

pub(crate) mod account;

use account::AccountMutation;

use async_graphql::MergedObject;


//------------------------------------------------------------------------------
/// Mutation root.
//------------------------------------------------------------------------------
#[derive(MergedObject, Default)]
pub(crate) struct MutationRoot(AccountMutation);
