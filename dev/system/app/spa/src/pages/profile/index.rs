//------------------------------------------------------------------------------
//! Profile.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::apis::graphql::post_graphql;
use crate::apis::graphql::account::{ Accounts, accounts };

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let state: &AppState = use_context(cx);
    let response = post_graphql::<Accounts>
    (
        state,
        "/account/graphql",
         accounts::Variables
    ).await;
    log::info!("response: {:?}", response);

    view!
    {
        cx,
        h1(class="ui_heading h1") { (t!("pages.profile.index.heading")) }
    }
}
