//------------------------------------------------------------------------------
//! Mypage.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Index<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        div
        {
            "mypage"
        }
    }
}
