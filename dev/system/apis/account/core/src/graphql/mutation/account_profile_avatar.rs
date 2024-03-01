//------------------------------------------------------------------------------
//! AccountProfileAvatar mutation.
//------------------------------------------------------------------------------

use async_graphql::{ Context, Object, Upload, Result };


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
        file: Upload,
    ) -> Result<bool>
    {
        println!("upload_avatar: {:?}", file.value(ctx).unwrap().filename);
        Ok(true)
    }
}
