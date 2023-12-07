//------------------------------------------------------------------------------
/// Installs test data for the application.
//------------------------------------------------------------------------------

use meower_migration::Migrator;
use meower_entity::user::ActiveModel as ActiveUser;
use meower_entity::account::ActiveModel as ActiveAccount;
use meower_core::Config;

use sea_orm_migration::MigratorTrait;
use sea_orm::{
    Database,
    ActiveModelTrait,
    ActiveValue,
    ActiveModelBehavior,
    TransactionTrait,
};

#[tokio::main]
async fn main()
{
    let config = Config::get();
    let hdb = Database::connect(config.database_url())
        .await
        .expect("Failed to setup the database");
    let tsx = hdb.begin().await.unwrap();

    // Refreshes the database.
    Migrator::refresh(&tsx).await.unwrap();

    // Creates a test user.
    let user = ActiveUser
    {
        user_id: ActiveValue::NotSet,
        email: ActiveValue::Set("dev.honda.ichigo@gmail.com".to_owned()),
        password: ActiveValue::Set("password123!".to_owned()),
        ..ActiveUser::new()
    };
    if let Ok(user) = user.insert(&tsx).await
    {
        let account = ActiveAccount
        {
            account_id: ActiveValue::NotSet,
            user_id: ActiveValue::Set(user.user_id),
            account_name: ActiveValue::Set("ichigo_dev".to_owned()),
            ..ActiveAccount::new()
        };
        account.insert(&tsx).await.unwrap();
    }

    tsx.commit().await.unwrap();
    println!("=== Test data installed ===");
}
