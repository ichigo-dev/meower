//------------------------------------------------------------------------------
//! Edit profile page.
//------------------------------------------------------------------------------

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn EditProfile<G: Html>( cx: Scope ) -> View<G>
{
    view!
    {
        cx,
        h1 { "Edit profile" }
        a(href="/mypage", rel="external") { "Mypage" }
    }
}
