//------------------------------------------------------------------------------
//! AccountProfileCover mutation.
//------------------------------------------------------------------------------

use async_graphql::{ Context, Object, Upload, Result };


//------------------------------------------------------------------------------
/// Mutation.
//------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct AccountProfileCoverMutation;

#[Object]
impl AccountProfileCoverMutation
{
    //--------------------------------------------------------------------------
    /// Uploads cover.
    //--------------------------------------------------------------------------
    async fn upload_cover
    (
        &self,
        ctx: &Context<'_>,
        file: Option<Upload>,
    ) -> Result<bool>
    {
        println!("upload_cover: {:?}", file);
        Ok(true)
    }
}
