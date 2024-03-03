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
    pub(crate) body_limit: usize,

    // JWT config.
    pub(crate) client_id_key: String,
    pub(crate) jwt_public_key: String,

    // Urls
    pub(crate) auth_api_url: String,
    pub(crate) account_api_url: String,

    // API config.
    pub(crate) auth_api_key_key: String,
    pub(crate) auth_api_key_val: String,
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
        let body_limit = env::var("BODY_LIMIT")
            .expect("BODY_LIMIT must be set")
            .parse::<usize>()
            .expect("BODY_LIMIT must be a number");

        // JWT config.
        let client_id_key = env::var("CLIENT_ID_KEY")
            .expect("CLIENT_ID_KEY must be set");
        let jwt_public_key = env::var("JWT_PUBLIC_KEY")
            .expect("JWT_PUBLIC_KEY must be set");

        // Urls.
        let auth_api_url = env::var("AUTH_API_URL")
            .expect("AUTH_API_URL must be set");
        let account_api_url = env::var("ACCOUNT_API_URL")
            .expect("ACCOUNT_API_URL must be set");

        // API config.
        let auth_api_key_key = env::var("AUTH_API_KEY_KEY")
            .expect("AUTH_API_KEY_KEY must be set");
        let auth_api_key_val = env::var("AUTH_API_KEY_VAL")
            .expect("AUTH_API_KEY_VAL must be set");

        Self
        {
            // Server config.
            port,
            body_limit,

            // JWT config.
            client_id_key,
            jwt_public_key,

            // Urls.
            auth_api_url,
            account_api_url,

            // API config.
            auth_api_key_key,
            auth_api_key_val,
        }
    }
}
