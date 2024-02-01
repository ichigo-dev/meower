//------------------------------------------------------------------------------
//! Account list components.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::utils::request_graphql::post_graphql;
use crate::components::GraphQLErrorAlert;

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Gets my accounts.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/pages/account/index/get_account_list.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct GetAccountList;


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
         {
             public_user_id: state.config.public_user_id.clone()
         },
    ).await;

    // Error handling.
    let response = match response
    {
        Ok(response) => response,
        Err(_) => return view! { cx, GraphQLErrorAlert() },
    };
    if let Some(_) = response.errors
    {
        return view! { cx, GraphQLErrorAlert() };
    }
    let data = match response.data
    {
        Some(data) => data,
        None => return view! { cx, GraphQLErrorAlert() },
    };

    // Gets accounts.
    let accounts = create_signal(cx, data.accounts);
    if accounts.get().len() <= 0
    {
        return view!
        {
            cx,
            div(class="ui_box surface radius padding_lg")
            {
                p(class="ui_text ui_text_size_md")
                {
                    (t!("pages.account.index.account_list.no_accounts"))
                }
            }
        };
    }

    view!
    {
        cx,
        ul(class="ui_list primary simple width_full")
        {
            Indexed
            (
                iterable=accounts,
                view=|cx, account|
                {
                    let href = format!("/account/{}", account.account_name);
                    let account_name = account.account_name.clone();
                    return view!
                    {
                        cx,
                        li(class="clickable padding_zero")
                        {
                            a
                            (
                                href=href,
                                rel="external",
                                class="display_block padding_vertical_sm padding_horizontal_md",
                            )
                            {
                                (account_name)
                            }
                        }
                    };
                }
            )
        }
    }
}
