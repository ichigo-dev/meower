//------------------------------------------------------------------------------
//! GraphQL mutation.
//------------------------------------------------------------------------------

mod account;
mod account_profile;
mod account_profile_avatar;
mod account_profile_cover;
mod group;
mod group_avatar;
mod group_cover;

use account::AccountMutation;
use account_profile::AccountProfileMutation;
use account_profile_avatar::AccountProfileAvatarMutation;
use account_profile_cover::AccountProfileCoverMutation;
use group::GroupMutation;
use group_avatar::GroupAvatarMutation;
use group_cover::GroupCoverMutation;

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
    GroupMutation,
    GroupAvatarMutation,
    GroupCoverMutation,
);
