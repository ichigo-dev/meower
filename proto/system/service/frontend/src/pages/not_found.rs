//------------------------------------------------------------------------------
//! NotFound page.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn NotFound<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div
        {
            "404 Not Found"
        }
    }
}