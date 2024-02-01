//------------------------------------------------------------------------------
//! Application state.
//------------------------------------------------------------------------------

use crate::Config;

use reqwest::Client;
use reqwest::header::HeaderMap;


//------------------------------------------------------------------------------
/// Application state.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub struct AppState
{
    pub config: Config,
    pub client: Client,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        let config = Config::init();
        let headers = HeaderMap::from_iter(vec!
        [
            (
                config.client_id_key.clone().parse().unwrap(),
                config.client_id.clone().parse().unwrap(),
            ),
        ]);
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or(Client::new());
        Self { config, client }
    }
}
