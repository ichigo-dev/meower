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
    pub(crate) debug_mode: bool,
    pub(crate) port: u16,

    // Database config.
    pub(crate) database_url: String,

    // Locale config.
    pub(crate) fallback_locale: String,

    // JWT config.
    pub(crate) jwt_issue: String,
    pub(crate) jwt_audience: Vec<String>,
    pub(crate) jwt_expiration_minutes: i64,
    pub(crate) jwt_secret: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        // Server config.
        let debug_mode = env::var("DEBUG_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("DEBUG_MODE must be a boolean");
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

        Self
        {
            debug_mode,
            port,
            database_url,
            fallback_locale,
            jwt_issue,
            jwt_audience,
            jwt_expiration_minutes,
            jwt_secret,
        }
    }
}
