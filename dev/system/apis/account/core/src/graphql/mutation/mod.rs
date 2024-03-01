//------------------------------------------------------------------------------
//! GraphQL mutation.
//------------------------------------------------------------------------------

mod account;
mod account_profile;
mod account_profile_avatar;
mod account_profile_cover;

use account::AccountMutation;
use account_profile::AccountProfileMutation;
use account_profile_avatar::AccountProfileAvatarMutation;
use account_profile_cover::AccountProfileCoverMutation;

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
    AccountProfileCoverMutation,
);
