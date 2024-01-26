//------------------------------------------------------------------------------
//! Configuration.
//------------------------------------------------------------------------------

use std::env;


//------------------------------------------------------------------------------
/// Config.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct Config
{
    // Server config.
    pub(crate) port: u16,

    // JWT config.
    pub(crate) client_id_key: String,
    pub(crate) jwt_public_key: String,

    // Urls
    pub(crate) auth_api_url: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        // Server config.
        let port = env::var("PORT")
            .expect("PORT must be set")
            .parse::<u16>()
            .expect("PORT must be a number");

        // JWT config.
        let client_id_key = env::var("CLIENT_ID_KEY")
            .expect("CLIENT_ID_KEY must be set");
        let jwt_public_key = env::var("JWT_PUBLIC_KEY")
            .expect("JWT_PUBLIC_KEY must be set");

        // Urls.
        let auth_api_url = env::var("AUTH_API_URL")
            .expect("AUTH_API_URL must be set");

        Self
        {
            // Server config.
            port,

            // JWT config.
            client_id_key,
            jwt_public_key,

            // Urls
            auth_api_url,
        }
    }
}
