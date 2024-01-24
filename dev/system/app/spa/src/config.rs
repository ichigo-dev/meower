//------------------------------------------------------------------------------
//! Configuration.
//------------------------------------------------------------------------------


//------------------------------------------------------------------------------
/// Config.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub(crate) struct Config
{
    pub(crate) api_url: String,
    pub(crate) client_id_key: String,
    pub(crate) client_id: String,
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
        let client_id = document
            .get_element_by_id(std::env!("CLIENT_ID_KEY"))
            .unwrap()
            .get_attribute("data-value")
            .unwrap_or("".to_string());
        let access_token = document
            .get_element_by_id(std::env!("ACCESS_TOKEN_KEY"))
            .unwrap()
            .get_attribute("data-value")
            .unwrap_or("".to_string());

        let api_url = std::env!("API_URL").to_string();
        let client_id_key = std::env!("CLIENT_ID_KEY").to_string();
        let client_id = client_id;
        let access_token = access_token;

        Self
        {
            api_url,
            client_id_key,
            client_id,
            access_token,
        }
    }
}
