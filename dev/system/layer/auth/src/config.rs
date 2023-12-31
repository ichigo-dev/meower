//------------------------------------------------------------------------------
//! Configuration.
//------------------------------------------------------------------------------

use std::env;


//------------------------------------------------------------------------------
/// Config.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub(crate) struct Config
{
    // System config.
    pub(crate) system_url: String,

    // Server config.
    pub(crate) debug_mode: bool,
    pub(crate) port: u16,
    pub(crate) proxy_url: String,
    pub(crate) provide_pages: bool,

    // Database config.
    pub(crate) database_url: String,

    // Locale config.
    pub(crate) fallback_locale: String,

    // JWT config.
    pub(crate) jwt_issue: String,
    pub(crate) jwt_audience: Vec<String>,
    pub(crate) jwt_expiration_minutes: i64,
    pub(crate) jwt_secret: String,

    // Email config.
    pub(crate) system_email_address: String,
    pub(crate) smtp_tls: bool,
    pub(crate) smtp_host: String,
    pub(crate) smtp_port: u16,
    pub(crate) smtp_user: String,
    pub(crate) smtp_password: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        // System config.
        let system_url = env::var("SYSTEM_URL")
            .expect("SYSTEM_URL must be set");

        // Server config.
        let debug_mode = env::var("DEBUG_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("DEBUG_MODE must be a boolean");
        let port = env::var("PORT")
            .expect("PORT must be set")
            .parse::<u16>()
            .expect("PORT must be a number");
        let proxy_url = env::var("PROXY_URL")
            .expect("PROXY_URL must be set")
            .trim_end_matches('/')
            .to_string();
        let provide_pages = env::var("PROVIDE_PAGES")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("PROVIDE_PAGES must be a boolean");

        // Database config.
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        // Locale config.
        let fallback_locale = env::var("FALLBACK_LOCALE")
            .unwrap_or_else(|_| "en".to_string());

        // JWT config.
        let jwt_issue = env::var("JWT_ISSUE")
            .expect("JWT_ISSUE must be set");
        let jwt_audience = env::var("JWT_AUDIENCE")
            .expect("JWT_AUDIENCE must be set")
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let jwt_expiration_minutes = env::var("JWT_EXPIRATION_MINUTES")
            .expect("JWT_EXPIRATION_MINUTES must be set")
            .parse::<i64>()
            .expect("JWT_EXPIRATION_MINUTES must be a number");
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        // Email config.
        let system_email_address = env::var("SYSTEM_EMAIL_ADDRESS")
            .expect("SYSTEM_EMAIL_ADDRESS must be set");
        let smtp_tls = env::var("SMTP_TLS")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("SMTP_TLS must be a boolean");
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

        Self
        {
            // System config.
            system_url,

            // Server config.
            debug_mode,
            port,
            proxy_url,
            provide_pages,

            // Database config.
            database_url,

            // Locale config.
            fallback_locale,

            // JWT config.
            jwt_issue,
            jwt_audience,
            jwt_expiration_minutes,
            jwt_secret,

            // Email config.
            system_email_address,
            smtp_tls,
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_password,
        }
    }
}
