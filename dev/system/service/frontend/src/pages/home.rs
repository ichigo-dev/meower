//------------------------------------------------------------------------------
//! Home page.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Home<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div
        {
            "home"
        }
    }
}