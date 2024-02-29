//------------------------------------------------------------------------------
//! GraphQL mutation.
//------------------------------------------------------------------------------

pub mod account;
pub mod account_profile;
pub mod account_profile_avatar;

use account::AccountMutation;
use account_profile::AccountProfileMutation;
use account_profile_avatar::AccountProfileAvatarMutation;

use async_graphql::MergedObject;


//------------------------------------------------------------------------------
/// Mutation root.
//------------------------------------------------------------------------------
#[derive(MergedObject, Default)]
pub(crate) struct MutationRoot
(
    AccountMutation,
    AccountProfileMutation,
    AccountProfileAvatarMutation,
);
