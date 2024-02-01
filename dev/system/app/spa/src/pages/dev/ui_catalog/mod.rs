//------------------------------------------------------------------------------
//! UI catalog page.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn UiCatalog<G: Html>() -> View<G>
{
    view!
    {
        Main
        (
            heading=t!("pages.dev.ui_catalog.heading"),
            children=view! { "UI catalog" }
        )
    }
}
