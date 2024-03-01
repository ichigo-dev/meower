//------------------------------------------------------------------------------
//! UploadFile.
//------------------------------------------------------------------------------

use reqwest::header::HeaderMap;


//------------------------------------------------------------------------------
/// UploadFile.
//------------------------------------------------------------------------------
#[derive(Clone, Debug, Default)]
pub struct UploadFile
{
    pub name: String,
    pub content: Vec<u8>,
    pub file_name: String,
    pub mime: String,
    pub headers: HeaderMap,
}
