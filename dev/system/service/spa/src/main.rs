//------------------------------------------------------------------------------
//! SPA frontend for the Meower.
//------------------------------------------------------------------------------

use rust_i18n::t;
use sycamore::prelude::*;

use log::debug;

// Loads the locales.
rust_i18n::i18n!("locales");


//------------------------------------------------------------------------------
/// Application root.
//------------------------------------------------------------------------------
#[component]
pub fn Root<G: Html>( cx: Scope ) -> View<G>
{
    // Sets the locale.
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let language = navigator.language().unwrap();
    rust_i18n::set_locale(language.as_str());

    view!
    {
        cx,
        (t!("hello"))
    }
}


//------------------------------------------------------------------------------
/// Application entry point.
//------------------------------------------------------------------------------
fn main()
{
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(Root);
}
