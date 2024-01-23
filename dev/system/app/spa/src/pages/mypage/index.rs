//------------------------------------------------------------------------------
//! Mypage.
//------------------------------------------------------------------------------

use crate::apis::mypage;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let profile = mypage::get_profile(cx).await;

    view!
    {
        cx,
        h1(class="ui_heading h1") { (t!("pages.mypage.index.heading")) }
    }
}
