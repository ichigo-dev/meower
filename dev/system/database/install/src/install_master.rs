//------------------------------------------------------------------------------
//! Master data installer.
//------------------------------------------------------------------------------

use meower_install::utils::install_master;
use meower_migration::Migrator;

use std::env;

use sea_orm_migration::MigratorTrait;
use sea_orm::{ Database, TransactionTrait };


//------------------------------------------------------------------------------
/// Installs master data for the application.
//------------------------------------------------------------------------------
#[tokio::main]
async fn main()
{
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let hdb = Database::connect(database_url.as_str())
        .await
        .expect("Failed to setup the database");

    println!("=== Installing master data ===");
    let tsx = hdb.begin().await.unwrap();

    // Refreshes the database.
    Migrator::refresh(&tsx).await.unwrap();

    // Installs master datas.
    if let Err(e) = install_master::install_master(&tsx).await
    {
        tsx.rollback().await.unwrap();
        panic!("{:?}", e);
    }

    tsx.commit().await.unwrap();
    println!("=== Master data installed ===");
}
