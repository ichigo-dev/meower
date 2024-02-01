//------------------------------------------------------------------------------
//! SPA frontend for the Meower.
//------------------------------------------------------------------------------

mod components;
mod features;
mod layouts;
mod pages;
mod root;
mod routes;
mod config;
mod state;
mod utils;

use root::Root;
pub use config::Config;
pub use state::AppState;

// Loads the locales.
rust_i18n::i18n!("locales");


//------------------------------------------------------------------------------
/// Application entry point.
//------------------------------------------------------------------------------
fn main()
{
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(Root);
}
