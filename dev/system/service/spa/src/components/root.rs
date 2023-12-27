//------------------------------------------------------------------------------
//! Application root component.
//------------------------------------------------------------------------------

use rust_i18n::t;
use sycamore::prelude::*;


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
