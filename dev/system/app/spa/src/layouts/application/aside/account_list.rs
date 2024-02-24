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
use sycamore::futures::{ create_resource, spawn_local_scoped };
use sycamore_router::navigate;


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

            spawn_local_scoped(async move
            {
                let mut state: AppState = use_context();
                let _ = post_graphql::<SelectAccount>
                (
                    &mut state,
                    "/account/graphql",
                     select_account::Variables
                     {
                         account_name: selected_account_name.get_clone(),
                     },
                ).await;
            });
        };
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

                    // List item states.
                    let selected = create_signal(false);
                    let clickable = create_signal(true);
                    let compared_account_name = account_name.clone();
                    create_effect(move ||
                    {
                        selected.set
                        (
                            compared_account_name
                                == selected_account_name.get_clone()
                        );
                        clickable.set(!selected.get());
                    });

                    let cloned_account_name = account_name.clone();
                    let cloned_name = name.clone();
                    let cloned_file_key = file_key.clone();

                    view!
                    {
                        ListItem
                        (
                            clickable=BoolProp(clickable.get()).into(),
                            on:click=move |_|
                            {
                                if selected.get()
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

                                let href = format!
                                (
                                    "/account/{}",
                                    &account_name
                                );
                                navigate(&href);
                            },
                        )
                        {
                            MiniProfile
                            (
                                name=cloned_name,
                                account_name=cloned_account_name,
                                file_key=cloned_file_key,
                                clickable=*clickable,
                                selected=*selected,
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
                    on:click=move |_|
                    {
                        navigate(&show_profile_href.get_clone());
                    },
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
                    on:click=move |_|
                    {
                        navigate("/account/create");
                    },
                )
                {
                    (t!("common.aside.account_menu_button.button.add_account"))
                }
            }
        }
    }
}
