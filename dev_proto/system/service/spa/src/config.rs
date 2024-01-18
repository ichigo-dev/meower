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
    pub(crate) auth_server_url: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        let api_url = std::env!("API_URL").to_string();
        let auth_server_url = std::env!("AUTH_SERVER_URL").to_string();

        Self
        {
            api_url,
            auth_server_url,
        }
    }
}
