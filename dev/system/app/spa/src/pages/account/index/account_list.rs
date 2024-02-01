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
pub async fn AccountList<G: Html>() -> View<G>
{
    let state: &AppState = use_context();
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
        Err(_) => return view! { GraphQLErrorAlert() },
    };
    if let Some(_) = response.errors
    {
        return view! { GraphQLErrorAlert() };
    }
    let data = match response.data
    {
        Some(data) => data,
        None => return view! { GraphQLErrorAlert() },
    };

    // Gets accounts.
    if data.accounts.len() <= 0
    {
        return view!
        {
            div(class="ui_box surface radius padding_lg")
            {
                p(class="ui_text ui_text_size_md")
                {
                    (t!("pages.account.index.account_list.no_accounts"))
                }
            }
        };
    }
    let accounts = create_signal(data.accounts);

    view!
    {
        ul(class="ui_list primary simple width_full")
        {
            Indexed
            (
                iterable=*accounts,
                view=|account|
                {
                    let href = format!("/account/{}", account.account_name);
                    let account_name = account.account_name.clone();
                    return view!
                    {
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
