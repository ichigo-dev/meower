//------------------------------------------------------------------------------
//! Shared state.
//------------------------------------------------------------------------------

use crate::Config;
use crate::graphql::{ QueryRoot, MutationRoot };

use async_graphql::{
    EmptySubscription,
    Schema,
};
use sea_orm::{ Database, DbConn };


//------------------------------------------------------------------------------
/// AppState.
//------------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct AppState
{
    pub(crate) config: Config,
    pub(crate) hdb: DbConn,
    pub(crate) schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
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
        let schema = Schema::build
            (
                QueryRoot::default(),
                MutationRoot,
                EmptySubscription
            )
            .finish();

        Self
        {
            config,
            hdb,
            schema,
        }
    }
}
