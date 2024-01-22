//------------------------------------------------------------------------------
//! Initial data installer.
//------------------------------------------------------------------------------

use meower_auth_entity::client_application::ActiveModel as ClientApplicationActiveModel;
use meower_entity_ext::ValidateExt;

use std::env;

use sea_orm::{ ActiveModelTrait, ActiveValue, Database, TransactionTrait };


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

    let meower_redirect_uri = env::var("MEOWER_REDIRECT_URI").unwrap();
    let app_url = env::var("APP_URL").unwrap();
    let client_application = ClientApplicationActiveModel
    {
        name: ActiveValue::Set("Meower".to_string()),
        url: ActiveValue::Set(app_url),
        redirect_uri: ActiveValue::Set(meower_redirect_uri),
        ..Default::default()
    };
    let client_application = match client_application
        .validate_and_insert(&tsx)
        .await
    {
        Ok(client_application) => client_application,
        Err(e) =>
        {
            tsx.rollback().await.unwrap();
            panic!("Failed to install initial data: {}", e.get_message());
        },
    };

    let mut client_application: ClientApplicationActiveModel
        = client_application.into();
    let meower_client_id = env::var("MEOWER_CLIENT_ID").unwrap();
    let meower_client_secret = env::var("MEOWER_CLIENT_SECRET").unwrap();
    client_application.client_id = ActiveValue::Set(meower_client_id);
    client_application.client_secret = ActiveValue::Set(meower_client_secret);
    client_application.update(&tsx).await.unwrap();

    println!("=== Initial data installed ===");

    tsx.commit().await.unwrap();
}
