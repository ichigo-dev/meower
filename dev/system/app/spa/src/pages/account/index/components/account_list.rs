//------------------------------------------------------------------------------
//! Account list components.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::apis::graphql::post_graphql;
use crate::components::GraphQLErrorAlert;

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Gets my accounts.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/account/schema.graphql",
    query_path = "graphql/account/get_account_list.graphql",
    response_derives = "Debug",
)]
pub struct GetAccountList;


//------------------------------------------------------------------------------
/// AccountList.
//------------------------------------------------------------------------------
#[component]
pub(crate) async fn AccountList<G: Html, 'cx>( cx: Scope<'cx> ) -> View<G>
{
    let state: &AppState = use_context(cx);
    let response = post_graphql::<GetAccountList>
    (
        state,
        "/account/graphql",
         get_account_list::Variables
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
