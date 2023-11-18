//------------------------------------------------------------------------------
//! Provides session layer.
//------------------------------------------------------------------------------

use std::env;

use tower_sessions::{ RedisStore, SessionManagerLayer };
use tower_sessions::fred::prelude::*;


//------------------------------------------------------------------------------
/// Creates session layer.
//------------------------------------------------------------------------------
pub(crate) async fn layer() -> SessionManagerLayer<RedisStore>
{
    // Creates Redis config.
    let redis_url = env::var("REDIS_URL")
        .unwrap_or("redis://redis:7000".to_string());
    let config = RedisConfig::from_url(&redis_url).unwrap();

    // Connects Redis server.
    let redis_client = Builder::from_config(config).build().unwrap();
    let _redis_conn = redis_client.connect();
    redis_client.wait_for_connect().await.unwrap();

    // Creates session service.
    let session_store = RedisStore::new(redis_client);
    SessionManagerLayer::new(session_store)
}
