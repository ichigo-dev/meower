//------------------------------------------------------------------------------
//! Account.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::apis::graphql::post_graphql;
use crate::apis::graphql::account::{ GetMyAccounts, get_my_accounts };

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let state: &AppState = use_context(cx);
    let _response = post_graphql::<GetMyAccounts>
    (
        state,
        "/account/graphql",
         get_my_accounts::Variables
    ).await;

    view!
    {
        cx,
        h1(class="ui_heading h1") { (t!("pages.account.index.heading")) }
    }
}
