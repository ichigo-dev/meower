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

    // Storage config.
    pub(crate) storage_url: String,
    pub(crate) storage_bucket: String,
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

        // Storage config.
        let storage_url = env::var("STORAGE_URL")
            .expect("STORAGE_URL must be set");
        let storage_bucket = env::var("STORAGE_BUCKET")
            .expect("STORAGE_BUCKET must be set");

        Self
        {
            // Server config.
            port,

            // Locale config.
            fallback_locale,

            // Database config.
            database_url,

            // Storage config.
            storage_url,
            storage_bucket,
        }
    }
}
