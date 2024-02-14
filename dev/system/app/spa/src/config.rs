//------------------------------------------------------------------------------
//! Configuration.
//------------------------------------------------------------------------------


//------------------------------------------------------------------------------
/// Config.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Config
{
    pub dev_mode: bool,
    pub app_url: String,
    pub api_url: String,
    pub client_id_key: String,
    pub client_id: String,
    pub public_user_id: String,
    pub access_token: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub fn init() -> Self
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

        let dev_mode = std::env!("DEV_MODE") == "true";
        let app_url = std::env!("APP_URL").to_string();
        let api_url = std::env!("API_URL").to_string();
        let client_id_key = std::env!("CLIENT_ID_KEY").to_string();

        Self
        {
            dev_mode,
            app_url,
            api_url,
            client_id_key,
            client_id,
            public_user_id,
            access_token,
        }
    }

    //--------------------------------------------------------------------------
    /// Updates the access token.
    //--------------------------------------------------------------------------
    pub fn update_access_token( &mut self, access_token: &str )
    {
        self.access_token = access_token.to_string();
    }
}
