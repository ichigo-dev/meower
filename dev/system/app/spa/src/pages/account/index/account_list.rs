//------------------------------------------------------------------------------
//! Account list components.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::request_graphql::post_graphql;
use crate::utils::props::*;
use crate::variables::*;

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
    let state: AppState = use_context();
    let response = post_graphql::<GetAccountList>
    (
        &state,
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
            Box(classes=StrProp("ui_box surface radius padding_lg").into())
            {
                Typography
                {
                    (t!("pages.account.index.account_list.no_accounts"))
                }
            }
        };
    }
    let accounts = create_signal(data.accounts);

    view!
    {
        List
        (
            color=Colors::Primary.into(),
            variant=ListVariant::Simple.into(),
        )
        {
            Indexed
            (
                iterable=*accounts,
                view=|account|
                {
                    let href = format!("/account/{}", account.account_name);
                    let account_name = account.account_name.clone();
                    view!
                    {
                        ListItem
                        (
                            clickable=BoolProp(true).into(),
                            classes=StrProp("padding_zero").into(),
                        )
                        {
                            Link
                            (
                                href=StringProp(href).into(),
                                classes=StrProp("display_block padding_vertical_sm padding_horizontal_md").into(),
                            )
                            {
                                (account_name)
                            }
                        }
                    }
                }
            )
        }
    }
}
