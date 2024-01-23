//------------------------------------------------------------------------------
//! Shared state.
//------------------------------------------------------------------------------

use crate::Config;

use sea_orm::{ Database, DbConn };


//------------------------------------------------------------------------------
/// AppState.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    pub(crate) config: Config,
    pub(crate) hdb: DbConn,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Initializes the application state.
    //--------------------------------------------------------------------------
    pub(crate) async fn init() -> Self
    {
        let config = Config::init();
        let hdb = Database::connect(&config.database_url).await.unwrap();
        Self
        {
            config,
            hdb,
        }
    }
}
