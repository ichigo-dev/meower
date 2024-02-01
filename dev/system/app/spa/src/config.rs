//------------------------------------------------------------------------------
//! Configuration.
//------------------------------------------------------------------------------

use std::sync::{ Arc, Mutex };


//------------------------------------------------------------------------------
/// Config.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub(crate) struct Config
{
    pub(crate) app_url: String,
    pub(crate) api_url: String,
    pub(crate) client_id_key: String,
    pub(crate) client_id: String,
    pub(crate) public_user_id: String,
    pub(crate) access_token: Arc<Mutex<String>>,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let client_id = document
            .get_element_by_id(std::env!("CLIENT_ID_KEY"))
            .unwrap()
            .get_attribute("data-value")
            .unwrap_or("".to_string());
        let public_user_id = document
            .get_element_by_id(std::env!("PUBLIC_USER_ID_KEY"))
            .unwrap()
            .get_attribute("data-value")
            .unwrap_or("".to_string());
        let access_token = document
            .get_element_by_id(std::env!("ACCESS_TOKEN_KEY"))
            .unwrap()
            .get_attribute("data-value")
            .unwrap_or("".to_string());

        let app_url = std::env!("APP_URL").to_string();
        let api_url = std::env!("API_URL").to_string();
        let client_id_key = std::env!("CLIENT_ID_KEY").to_string();
        let client_id = client_id;
        let access_token = Arc::new(Mutex::new(access_token));

        Self
        {
            app_url,
            api_url,
            client_id_key,
            client_id,
            public_user_id,
            access_token,
        }
    }
}
