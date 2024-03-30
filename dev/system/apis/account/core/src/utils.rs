//------------------------------------------------------------------------------
//! Utilities.
//------------------------------------------------------------------------------

use rust_i18n::t;
use base64::prelude::*;


//------------------------------------------------------------------------------
/// Base64 content.
//------------------------------------------------------------------------------
pub struct Base64Content
{
    pub content_type: String,
    pub binary: Vec<u8>,
    pub file_size: i64,
}


//------------------------------------------------------------------------------
/// Parses the base64 content.
//------------------------------------------------------------------------------
pub fn parse_base64( base64: &str ) -> Result<Base64Content, String>
{
    let (prefix, base64) = match base64.split_once(",")
    {
        Some((content_type, base64)) => (content_type, base64),
        None => return Err(t!("system.error.invalid_format").into()),
    };
    let content_type = prefix
        .split(";")
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap();
    let binary = BASE64_STANDARD.decode(base64.as_bytes()).unwrap();
    let file_size = binary.len().try_into().unwrap_or_default();

    let base64_content = Base64Content
    {
        content_type: content_type.to_string(),
        binary,
        file_size,
    };

    Ok(base64_content)
}
