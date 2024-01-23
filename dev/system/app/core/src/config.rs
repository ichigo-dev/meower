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
    pub(crate) jwt_user_token_key: String,
    pub(crate) jwt_access_token_key: String,
    pub(crate) jwt_public_key: String,
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
        let jwt_user_token_key = "jwt_user_token".to_string();
        let jwt_access_token_key = "jwt_access_token".to_string();
        let jwt_public_key = env::var("JWT_PUBLIC_KEY")
            .expect("JWT_PUBLIC_KEY must be set");

        Self
        {
            // Server config.
            port,

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
            jwt_user_token_key,
            jwt_access_token_key,
            jwt_public_key,
        }
    }
}
