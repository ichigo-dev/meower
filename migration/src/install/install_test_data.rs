//------------------------------------------------------------------------------
/// Installs test data for the application.
//------------------------------------------------------------------------------

use meower_migration::Migrator;
use meower_entity::user;
use meower_utility::Config;

use sea_orm_migration::MigratorTrait;
use sea_orm::{ Database, ActiveModelTrait, Set };
use sea_orm::ActiveValue::NotSet;

#[tokio::main]
async fn main()
{
    let config = Config::init();
    let hdb = Database::connect(config.database_url())
        .await
        .expect("Failed to setup the database");

    // Refreshes the database.
    Migrator::refresh(&hdb).await.unwrap();

    // Creates a test user.
    let user = user::ActiveModel
    {
        id: NotSet,
        email: Set("dev.honda.ichigo@gmail.com".to_owned()),
        account_name: Set("ichigo_dev".to_owned()),
        password: Set("password123!".to_owned()),
    };
    user.save(&hdb).await.unwrap();

    println!("=== Test data installed ===");
}
