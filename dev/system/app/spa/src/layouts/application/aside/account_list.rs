//------------------------------------------------------------------------------
//! Aside account list.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::request_graphql::post_graphql;
use crate::utils::props::*;
use crate::variables::*;

use graphql_client::GraphQLQuery;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Gets acount list.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/query/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct GetAccountList;


//------------------------------------------------------------------------------
/// AccountList.
//------------------------------------------------------------------------------
#[component]
pub async fn AccountList<G: Html>() -> View<G>
{
    let mut state: AppState = use_context();
    let config = state.config.clone();
    let public_user_id = config.public_user_id.clone();
    let data = match post_graphql::<GetAccountList>
    (
        &mut state,
        "/account/graphql",
         get_account_list::Variables
         {
             public_user_id: public_user_id,
         },
    ).await
    {
        Ok(data) => data,
        Err(e) => return view! { Alert { (e) } },
    };
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
