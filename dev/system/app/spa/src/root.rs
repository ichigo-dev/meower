//------------------------------------------------------------------------------
//! Application root component.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::Layout;
use crate::routes::AppRouter;
use crate::state::AppState;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Application root.
//------------------------------------------------------------------------------
#[component]
pub fn Root<G: Html>() -> View<G>
{
    // Sets the locale.
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let language = navigator.language().unwrap();
    rust_i18n::set_locale(language.as_str());

    // Initializes the application state.
    let app_state = AppState::new();
    provide_context(app_state);

    view!
    {
        SnackbarProvider
        {
            Layout
            {
                AppRouter
            }
        }
    }
}
