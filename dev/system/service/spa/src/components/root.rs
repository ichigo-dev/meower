//------------------------------------------------------------------------------
//! Application root component.
//------------------------------------------------------------------------------

use crate::layouts::Layout;
use crate::routes::AppRouter;
use crate::state::AppState;

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

    // Initializes the application state.
    let app_state = AppState::new();
    provide_context(cx, app_state);

    view!
    {
        cx,
        Layout(child=view!{ cx, AppRouter })
    }
}
