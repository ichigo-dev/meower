//------------------------------------------------------------------------------
//! Main entry point for the migration binary.
//------------------------------------------------------------------------------

use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main()
{
    cli::run_cli(meower_migration::Migrator).await;
}
