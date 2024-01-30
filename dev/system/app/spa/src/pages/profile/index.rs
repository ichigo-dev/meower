//------------------------------------------------------------------------------
//! Profile.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::apis::post;
use crate::apis::graphql::account::{ Accounts, accounts };

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn Index<'cx, G: Html>( cx: Scope<'cx> ) -> View<G>
{
    let state: &AppState = use_context(cx);
    let response = post
    (
        state,
        "/account/graphql",
        Accounts::build_query(accounts::Variables).query
    ).await;
    log::info!("response: {:?}", response);

    view!
    {
        cx,
        h1(class="ui_heading h1") { (t!("pages.profile.index.heading")) }
    }
}
