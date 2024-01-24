//------------------------------------------------------------------------------
//! Shared state.
//------------------------------------------------------------------------------

use crate::Config;

use std::sync::Arc;
use tokio::sync::RwLock;


//------------------------------------------------------------------------------
/// AppState.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    pub(crate) config: Config,
    pub(crate) cors_origins: Arc<RwLock<Vec<String>>>,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Initializes the application state.
    //--------------------------------------------------------------------------
    pub(crate) fn init() -> Self
    {
        let config = Config::init();
        let cors_origins = Arc::new(RwLock::new(vec!["*".to_string()]));
        Self
        {
            config,
            cors_origins,
        }
    }
}
