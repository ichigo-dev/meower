//------------------------------------------------------------------------------
//! SPA frontend for the Meower.
//------------------------------------------------------------------------------

mod apis;
mod components;
mod features;
mod layouts;
mod pages;
mod routes;
mod state;

use components::root::Root;
pub(crate) use state::AppState;

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
