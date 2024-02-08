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

    // Locale config.
    pub(crate) fallback_locale: String,

    // Database config.
    pub(crate) database_url: String,
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

        // Locale config.
        let fallback_locale = env::var("FALLBACK_LOCALE")
            .expect("FALLBACK_LOCALE must be set");

        // Database config.
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self
        {
            // Server config.
            port,

            // Locale config.
            fallback_locale,

            // Database config.
            database_url,
        }
    }
}
