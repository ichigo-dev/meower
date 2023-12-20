//------------------------------------------------------------------------------
/// Installs master data for the application.
//------------------------------------------------------------------------------

use meower_install::common::install_master;
use meower_migration::Migrator;
use meower_core::{ Config, I18n };

use sea_orm_migration::MigratorTrait;
use sea_orm::{ Database, TransactionTrait };

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

    // Installs master datas.
    if let Err(_) = install_master::install_master(&tsx, &i18n).await
    {
        tsx.rollback().await.unwrap();
        panic!("Failed to install master data");
    }

    tsx.commit().await.unwrap();
    println!("=== Master data installed ===");
}
