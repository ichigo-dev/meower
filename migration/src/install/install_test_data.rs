//------------------------------------------------------------------------------
/// Installs test data for the application.
//------------------------------------------------------------------------------

use meower_migration::Migrator;
use meower_entity::user;

pub use sea_orm_migration::MigratorTrait;
use sea_orm::{ Database, ActiveModelTrait, Set };
use sea_orm::ActiveValue::NotSet;

#[tokio::main]
async fn main()
{
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let hdb = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    // Refreshes the database.
    Migrator::refresh(&hdb).await.unwrap();

    // Creates a test user.
    let user = user::ActiveModel
    {
        id: NotSet,
        email: Set("dev.honda.ichigo@gmail.com".to_owned()),
        account_name: Set("ichigo-dev".to_owned()),
        password: Set("password".to_owned()),
    };
    user.save(&hdb).await.unwrap();

    println!("=== Test data installed ===");
}
