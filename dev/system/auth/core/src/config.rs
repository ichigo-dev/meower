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

    // Locale config.
    pub(crate) fallback_locale: String,

    // Parameter config.
    pub(crate) client_id_key: String,
    pub(crate) client_secret_key: String,

    // Mail config.
    pub(crate) smtp_host: String,
    pub(crate) smtp_port: u16,
    pub(crate) smtp_user: String,
    pub(crate) smtp_password: String,
    pub(crate) smtp_tls: bool,
    pub(crate) system_email_address: String,
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

        // Locale config.
        let fallback_locale = env::var("FALLBACK_LOCALE")
            .unwrap_or_else(|_| "en".to_string());

        // Parameter config.
        let client_id_key = env::var("CLIENT_ID_KEY")
            .expect("CLIENT_ID_KEY must be set");
        let client_secret_key = env::var("CLIENT_SECRET_KEY")
            .expect("CLIENT_SECRET_KEY must be set");

        // Mail config.
        let smtp_host = env::var("SMTP_HOST")
            .expect("SMTP_HOST must be set");
        let smtp_port = env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse::<u16>()
            .expect("SMTP_PORT must be a number");
        let smtp_user = env::var("SMTP_USER")
            .expect("SMTP_USER must be set");
        let smtp_password = env::var("SMTP_PASSWORD")
            .expect("SMTP_PASSWORD must be set");
        let smtp_tls = env::var("SMTP_TLS")
            .expect("SMTP_TLS must be set")
            .parse::<bool>()
            .expect("SMTP_TLS must be a boolean");
        let system_email_address = env::var("SYSTEM_EMAIL_ADDRESS")
            .expect("SYSTEM_EMAIL_ADDRESS must be set");

        Self
        {
            // Server config.
            port,

            // Database config.
            database_url,

            // Locale config.
            fallback_locale,

            // Parameter config.
            client_id_key,
            client_secret_key,

            // Mail config.
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_password,
            smtp_tls,
            system_email_address,
        }
    }
}
