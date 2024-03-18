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
use casbin::prelude::*;
use object_store::ObjectStore;
use object_store::local::LocalFileSystem;
use sea_orm::{ Database, DbConn };
use sqlx::postgres::PgPoolOptions;
use sqlx_adapter::SqlxAdapter;
use tokio::sync::RwLock;


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
    pub(crate) enforcer: Arc<RwLock<Enforcer>>,
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

        // GraphQL.
        let schema = Schema::build
        (
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription
        )
        .finish();

        // Storage.
        let storage = LocalFileSystem::new_with_prefix(&config.storage_bucket)
            .unwrap();
        let storage: Arc<Box<dyn ObjectStore>> = Arc::new(Box::new(storage));

        // Casbin.
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .unwrap();
        let model = DefaultModel::from_file("authorization/model.conf")
            .await
            .unwrap();
        let adapter = SqlxAdapter::new_with_pool(pool)
            .await
            .unwrap();
        let enforcer = Arc::new(RwLock::new
        (
            Enforcer::new(model, adapter).await.unwrap()
        ));

        Self
        {
            config,
            hdb,
            schema,
            storage,
            enforcer,
        }
    }
}
