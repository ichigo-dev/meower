//------------------------------------------------------------------------------
//! Aside account list.
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
/// Component.
//------------------------------------------------------------------------------
#[component]
pub async fn AccountList<G: Html>() -> View<G>
{
    let mut state: AppState = use_context();
    let selected_account = state.selected_account.get_clone();
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

    let show_profile_href = if let Some(selected_account) = selected_account
    {
        format!("/account/{}", selected_account.account_name)
    }
    else
    {
        "/account/create".to_string()
    };

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
                    let account_name = account.account_name.clone();
                    let mut name = "".to_string();
                    let mut file_key = "".to_string();

                    if let Some(profile) = account.default_account_profile
                    {
                        name = profile.name.clone();
                        if let Some(avatar) = profile.avatar
                        {
                            file_key = avatar.file_key.clone();
                        }
                    };

                    view!
                    {
                        ListItem(clickable=BoolProp(true).into())
                        {
                            MiniProfile
                            (
                                name=name,
                                account_name=account_name,
                                file_key=file_key,
                            )
                        }
                    }
                }
            )
            ListItem
            (
                clickable=BoolProp(true).into(),
                classes=StrProp("border_top padding_zero").into(),
            )
            {
                Button
                (
                    classes=StrProp("width_full").into(),
                    color=Colors::Transparent.into(),
                    href=OptionProp(Some(show_profile_href)).into(),
                )
                {
                    (t!("common.aside.account_menu_button.button.show_profile"))
                }
            }
            ListItem
            (
                clickable=BoolProp(true).into(),
                classes=StrProp("border_top padding_zero").into(),
            )
            {
                Button
                (
                    classes=StrProp("width_full").into(),
                    color=Colors::Transparent.into(),
                )
                {
                    (t!("common.aside.account_menu_button.button.add_account"))
                }
            }
        }
    }
}
