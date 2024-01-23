//------------------------------------------------------------------------------
//! Configuration.
//------------------------------------------------------------------------------

use std::collections::HashMap;

use wasm_bindgen::JsCast;


//------------------------------------------------------------------------------
/// Config.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub(crate) struct Config
{
    pub(crate) api_url: String,
    pub(crate) client_id: String,
    pub(crate) user_token: String,
    pub(crate) access_token: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let html_document = document
            .dyn_into::<web_sys::HtmlDocument>()
            .unwrap();
        let cookie = html_document.cookie().unwrap();
        let cookies = cookie
            .split("; ")
            .collect::<Vec<_>>()
            .iter()
            .map(|cookie| cookie.split("=").collect::<Vec<_>>())
            .map(|cookie| (cookie[0].to_string(), cookie[1].to_string()))
            .collect::<HashMap<_, _>>();

        let api_url = std::env!("API_URL").to_string();
        let client_id = cookies.get("client_id").unwrap_or_default();
        let user_token = cookies.get("user_token").unwrap_or_default();
        let access_token = cookies.get("access_token").unwrap_or_default();

        Self
        {
            api_url,
            client_id,
            user_token,
            access_token,
        }
    }
}
