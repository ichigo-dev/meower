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
    pub(crate) port: u16,
    pub(crate) database_url: String,
    pub(crate) fallback_locale: String,
}

impl Config
{
    //--------------------------------------------------------------------------
    /// Initializes the configuration.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a number");

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let fallback_locale = env::var("FALLBACK_LOCALE")
            .unwrap_or_else(|_| "en".to_string());

        Self
        {
            port,
            database_url,
            fallback_locale,
        }
    }
}
