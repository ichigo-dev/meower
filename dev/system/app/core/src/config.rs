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
    pub(crate) user_token_key: String,

    // Database config.
    pub(crate) database_url: String,

    // Urls.
    pub(crate) auth_login_url: String,
    pub(crate) auth_request_token_url: String,

    // JWT config.
    pub(crate) client_id_key: String,
    pub(crate) client_secret_key: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) access_token_key: String,
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
        let user_token_key = env::var("USER_TOKEN_KEY")
            .expect("JWT_ACCESS_TOKEN_KEY must be set");

        // Database config.
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        // Urls.
        let auth_login_url = env::var("AUTH_LOGIN_URL")
            .expect("AUTH_LOGIN_URL must be set");
        let auth_request_token_url = env::var("AUTH_REQUEST_TOKEN_URL")
            .expect("AUTH_REQUEST_TOKEN_URL must be set");

        // JWT config.
        let client_id_key = env::var("CLIENT_ID_KEY")
            .expect("CLIENT_ID_KEY must be set");
        let client_secret_key = env::var("CLIENT_SECRET_KEY")
            .expect("CLIENT_SECRET_KEY must be set");
        let client_id = env::var("CLIENT_ID")
            .expect("CLIENT_ID must be set");
        let client_secret = env::var("CLIENT_SECRET")
            .expect("CLIENT_SECRET must be set");
        let access_token_key = env::var("ACCESS_TOKEN_KEY")
            .expect("JWT_ACCESS_TOKEN_KEY must be set");

        Self
        {
            // Server config.
            port,
            user_token_key,

            // Database config.
            database_url,

            // Urls.
            auth_login_url,
            auth_request_token_url,

            // JWT config.
            client_id_key,
            client_secret_key,
            client_id,
            client_secret,
            access_token_key,
        }
    }
}
