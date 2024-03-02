//------------------------------------------------------------------------------
//! AccountProfileCover mutation.
//------------------------------------------------------------------------------

use async_graphql::{ Context, Object, Result };


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
        account_profile_token: String,
        base64: Option<String>,
    ) -> Result<bool>
    {
        println!("upload_cover: {:?}", base64);
        Ok(true)
    }
}
