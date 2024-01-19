//------------------------------------------------------------------------------
//! Initial data installer.
//------------------------------------------------------------------------------

use meower_auth_entity::client_application::ActiveModel as ClientApplicationActiveModel;
use meower_entity_ext::ValidateExt;

use std::env;

use sea_orm::{ ActiveValue, Database, TransactionTrait };


//------------------------------------------------------------------------------
/// Installs initial data for the application.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let hdb = Database::connect(database_url.as_str())
        .await
        .expect("Failed to setup the database");
    let tsx = hdb.begin().await.unwrap();

    println!("=== Installing initial data ===");

    let client_application = ClientApplicationActiveModel
    {
        name: ActiveValue::Set("Meower".to_string()),
        redirect_uri: ActiveValue::Set("http://localhost:8080/callback_login".to_string()),
        ..Default::default()
    };
    if let Err(e) = client_application.validate_and_insert(&tsx).await
    {
        tsx.rollback().await.unwrap();
        panic!("Failed to install initial data: {}", e.get_message());
    };

    println!("=== Initial data installed ===");

    tsx.commit().await.unwrap();
}
