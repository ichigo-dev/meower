//------------------------------------------------------------------------------
//! Shared state.
//------------------------------------------------------------------------------

use crate::Config;
use crate::graphql::{ QueryRoot, MutationRoot };

use std::sync::Arc;

use async_graphql::{
    EmptySubscription,
    Schema,
};
use object_store::ObjectStore;
use object_store::local::LocalFileSystem;
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
    pub(crate) storage: Arc<Box<dyn ObjectStore>>,
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
            MutationRoot::default(),
            EmptySubscription
        )
        .finish();

        let _ = config.storage_url;
        let storage = LocalFileSystem::new_with_prefix(&config.storage_bucket)
            .unwrap();
        let storage: Arc<Box<dyn ObjectStore>> = Arc::new(Box::new(storage));

        Self
        {
            config,
            hdb,
            schema,
            storage,
        }
    }
}
