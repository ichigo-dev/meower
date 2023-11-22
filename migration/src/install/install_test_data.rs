//------------------------------------------------------------------------------
/// Installs test data for the application.
//------------------------------------------------------------------------------

use meower_migration::Migrator;
use meower_entity::user;

use std::env;

pub use sea_orm_migration::MigratorTrait;
use sea_orm::{ Database, ActiveModelTrait, Set };
use sea_orm::ActiveValue::NotSet;
use argon2::{ self, Argon2, PasswordHasher };
use argon2::password_hash::SaltString;

#[tokio::main]
async fn main()
{
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let hdb = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    // Refreshes the database.
    Migrator::refresh(&hdb).await.unwrap();

    // Initializes the Argon2 hasher.
    let salt = env::var("ARGON2_PHC_SALT")
        .unwrap_or("salt".to_string());
    let salt_string = SaltString::from_b64(&salt)
        .unwrap();
    let argon2 = Argon2::new
    (
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::default(),
    );

    // Creates a test user.
    let password = "password";
    let hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .unwrap()
        .to_string();
    let user = user::ActiveModel
    {
        id: NotSet,
        email: Set("dev.honda.ichigo@gmail.com".to_owned()),
        account_name: Set("ichigo-dev".to_owned()),
        password: Set(hash.to_owned()),
    };
    user.save(&hdb).await.unwrap();

    println!("=== Test data installed ===");
}
