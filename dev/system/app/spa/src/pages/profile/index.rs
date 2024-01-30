//------------------------------------------------------------------------------
//! Profile.
//------------------------------------------------------------------------------

use crate::apis::get;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let state = use_context(cx);
    log::info!("{:?}", get(&state, "account/graphql").await);

    view!
    {
        cx,
        h1(class="ui_heading h1") { (t!("pages.profile.index.heading")) }
    }
}
