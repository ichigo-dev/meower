//------------------------------------------------------------------------------
//! Aside account list.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::types::SelectedAccount;
use crate::utils::request_graphql::post_graphql;
use crate::utils::props::*;
use crate::variables::*;

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::create_resource;


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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct SelectAccount;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub async fn AccountList<G: Html>( open: Signal<bool> ) -> View<G>
{
    let state: AppState = use_context();
    let config = state.config.clone();
    let public_user_id = config.public_user_id.clone();
    let accounts = create_signal(Vec::new());

    create_resource(async move
    {
        let mut state: AppState = use_context();
        if let Ok(data) = post_graphql::<GetAccountList>
        (
            &mut state,
            "/account/graphql",
             get_account_list::Variables
             {
                 public_user_id: public_user_id,
             },
        ).await
        {
            accounts.set(data.accounts);
        };
    });

    let show_profile_href = create_signal("/account/create".to_string());
    let selected_account_name = create_signal(String::new());

    create_effect(move ||
    {
        let selected_account = state.selected_account.get_clone();
        if let Some(selected_account) = selected_account
        {
            show_profile_href.set
            (
                format!
                (
                    "/account/{}",
                    selected_account.account_name
                )
            );
            selected_account_name.set(selected_account.account_name);
        };

        /*
            let data = match post_graphql::<SelectAccount>
            (
                &mut state,
                "/account/graphql",
                 select_account::Variables
                 {
                     account_name: account_name.clone(),
                 },
            ).await
            {
                Ok(data) => data,
                Err(e) => return,
            };
            let account = create_signal(data.select_account);
        */
    });

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
                view=move |account|
                {
                    let account_name = account.account_name.clone();
                    let selected = account_name == selected_account_name.get_clone();
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

                    let cloned_account_name = account_name.clone();
                    let cloned_name = name.clone();
                    let cloned_file_key = file_key.clone();

                    view!
                    {
                        ListItem
                        (
                            clickable=BoolProp(!selected).into(),
                            on:click=move |_|
                            {
                                if selected
                                {
                                    return;
                                }
                                open.set(false);

                                let selected_account = SelectedAccount
                                {
                                    account_name: account_name.clone(),
                                    name: name.clone(),
                                    avatar_file_key: file_key.clone(),
                                };
                                state.selected_account.set
                                (
                                    Some(selected_account)
                                );
                            },
                        )
                        {
                            MiniProfile
                            (
                                name=cloned_name,
                                account_name=cloned_account_name,
                                file_key=cloned_file_key,
                                clickable=!selected,
                                selected=selected,
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
                    href=OptionProp(Some(show_profile_href.get_clone())).into(),
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
                    href=OptionProp(Some("/account/create".to_string())).into(),
                )
                {
                    (t!("common.aside.account_menu_button.button.add_account"))
                }
            }
        }
    }
}
