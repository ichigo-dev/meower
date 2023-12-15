//------------------------------------------------------------------------------
/// Installs test data for the application.
//------------------------------------------------------------------------------

use meower_migration::Migrator;
use meower_core::{ Config, I18n };
use meower_entity::Validate;
use meower_entity::user::ActiveModel as ActiveUser;
use meower_entity::user_auth::ActiveModel as ActiveUserAuth;
use meower_entity::user_account::ActiveModel as ActiveUserAccount;
use meower_entity::user_account_profile::ActiveModel as ActiveUserAccountProfile;

use sea_orm_migration::MigratorTrait;
use sea_orm::{
    Database,
    ActiveValue,
    TransactionTrait,
};

#[tokio::main]
async fn main()
{
    let config = Config::new();
    let mut i18n = I18n::new();
    i18n.init("en", &config);

    let hdb = Database::connect(config.get("database.url"))
        .await
        .expect("Failed to setup the database");
    let tsx = hdb.begin().await.unwrap();

    // Refreshes the database.
    Migrator::refresh(&tsx).await.unwrap();

    for i in 1..=10
    {
        let user = ActiveUser
        {
            email: ActiveValue::set(format!("user{}@example.com", i)),
            ..Default::default()
        };
        let user = match user.validate_and_insert(&tsx, &i18n).await
        {
            Ok(user) => user,
            Err(_) =>
            {
                tsx.rollback().await.unwrap();
                panic!("Failed to insert `user` test data");
            }
        };
        let user_id = user.user_id;

        let user_auth = ActiveUserAuth
        {
            user_id: ActiveValue::Set(user_id),
            password: ActiveValue::set("password123!".to_string()),
            ..Default::default()
        };
        if user_auth.validate_and_insert(&tsx, &i18n).await.is_err()
        {
            tsx.rollback().await.unwrap();
            panic!("Failed to insert `user_auth` test data");
        }

        let user_account = ActiveUserAccount
        {
            user_id: ActiveValue::Set(user_id),
            user_account_name: ActiveValue::set(format!("user{}", i)),
            display_name: ActiveValue::set(format!("User {}", i)),
            ..Default::default()
        };
        let user_account = match user_account
            .validate_and_insert(&tsx, &i18n)
            .await
        {
            Ok(user_account) => user_account,
            Err(_) =>
            {
                tsx.rollback().await.unwrap();
                panic!("Failed to insert `user_account` test data");
            }
        };
        let user_account_id = user_account.user_account_id;

        let user_account_profile = ActiveUserAccountProfile
        {
            user_account_id: ActiveValue::Set(user_account_id),
            name: ActiveValue::set(format!("User {}", i)),
            biography: ActiveValue::set("Hello, world!".to_string()),
            company: ActiveValue::set("Example Inc.".to_string()),
            title: ActiveValue::set("Software Engineer".to_string()),
            location: ActiveValue::set("Tokyo, Japan".to_string()),
            ..Default::default()
        };
        if user_account_profile.validate_and_insert(&tsx, &i18n).await.is_err()
        {
            tsx.rollback().await.unwrap();
            panic!("Failed to insert `user_account_profile` test data");
        }
    }

    tsx.commit().await.unwrap();
    println!("=== Test data installed ===");
}
