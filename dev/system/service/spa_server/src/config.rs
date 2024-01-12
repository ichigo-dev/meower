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
    pub(crate) port: u16,

    // JWT config.
    pub(crate) jwt_audience: Vec<String>,
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
        let port = env::var("PORT")
            .expect("PORT must be set")
            .parse::<u16>()
            .expect("PORT must be a number");

        // JWT config.
        let jwt_audience = env::var("JWT_AUDIENCE")
            .expect("JWT_AUDIENCE must be set")
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        Self
        {
            port,

            // JWT config.
            jwt_audience,
            jwt_secret,
        }
    }
}
