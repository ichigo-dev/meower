//------------------------------------------------------------------------------
//! Dev page.
//------------------------------------------------------------------------------

use crate::layouts::application::Main;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<G: Html>() -> View<G>
{
    view!
    {
        Main
        (
            heading=t!("pages.dev.index.heading"),
            children=view!
            {
                a
                (
                    href="/dev/ui_catalog",
                    rel="external",
                    class="ui_button primary",
                )
                {
                    (t!("pages.dev.index.button.ui_catalog"))
                }
            }
        )
    }
}
