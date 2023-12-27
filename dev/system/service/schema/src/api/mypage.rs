//------------------------------------------------------------------------------
//! Mypage APIs.
//------------------------------------------------------------------------------

use serde::{ Serialize, Deserialize };


//------------------------------------------------------------------------------
/// Gets profile.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetProfileResponse
{
    pub name: String,
}
