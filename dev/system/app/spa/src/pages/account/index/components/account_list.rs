//------------------------------------------------------------------------------
//! Account list components.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::apis::graphql::{ Error, post_graphql };
use crate::apis::graphql::account::{ GetMyAccounts, get_my_accounts };
use crate::components::GraphQLErrorAlert;

use graphql_client::Response;
use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// AccountList.
//------------------------------------------------------------------------------
#[component]
pub(crate) async fn AccountList<G: Html, 'cx>( cx: Scope<'cx> ) -> View<G>
{
    let state: &AppState = use_context(cx);
    let response = post_graphql::<GetMyAccounts>
    (
        state,
        "/account/graphql",
         get_my_accounts::Variables
    ).await;

    let response = match response
    {
        Ok(response) => response,
        Err(_) => return view! { cx, GraphQLErrorAlert() },
    };

    if let Some(errors) = response.errors
    {
        return view! { cx, GraphQLErrorAlert() };
    }

    let data = match response.data
    {
        Some(data) => data,
        None => return view! { cx, GraphQLErrorAlert() },
    };
    log::info!("{:#?}", data.accounts);

    view!
    {
        cx,
        div { "AccountList" }
    }
}
