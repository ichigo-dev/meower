//------------------------------------------------------------------------------
//! AccountProfileAvatar mutation.
//------------------------------------------------------------------------------

use async_graphql::{ Context, Object, Result };


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountProfileAvatarMutation;

#[Object]
impl AccountProfileAvatarMutation
{
    //--------------------------------------------------------------------------
    /// Uploads avatar.
    //--------------------------------------------------------------------------
    async fn upload_avatar
    (
        &self,
        ctx: &Context<'_>,
        base64: String,
    ) -> Result<bool>
    {
        println!("upload_avatar: {:?}", base64);
        Ok(true)
    }
}
