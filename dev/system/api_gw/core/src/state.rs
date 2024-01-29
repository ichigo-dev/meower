//------------------------------------------------------------------------------
//! Shared state.
//------------------------------------------------------------------------------

use crate::Config;

use std::sync::Arc;

use reqwest::Client;
use tokio::sync::RwLock;
use serde::Deserialize;


//------------------------------------------------------------------------------
/// AppState.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    pub(crate) config: Config,
    pub(crate) cors_origins: Arc<RwLock<Vec<String>>>,
}


//------------------------------------------------------------------------------
/// AllowOrigins.
//------------------------------------------------------------------------------
#[derive(Debug, Deserialize, Default)]
pub(crate) struct AllowOrigins
{
    origins: Vec<String>,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Initializes the application state.
    //--------------------------------------------------------------------------
    pub(crate) async fn init() -> Self
    {
        let config = Config::init();

        // Gets allow origins.
        let client = Client::new();
        let endpoint = format!
        (
            "{}/api/client_application/get_allow_origins",
            config.auth_api_url,
        );
        let res = client
            .get(&endpoint)
            .header(&config.auth_api_key_key, &config.auth_api_key_val)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let allow_origins: AllowOrigins = serde_json::from_str(&res).unwrap();
        let cors_origins = Arc::new(RwLock::new(allow_origins.origins));
        Self
        {
            config,
            cors_origins,
        }
    }
}
