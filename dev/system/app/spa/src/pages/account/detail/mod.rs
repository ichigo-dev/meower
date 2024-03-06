//------------------------------------------------------------------------------
//! Account detail page.
//------------------------------------------------------------------------------

mod account_profiles;
mod account_profile_card;

use account_profiles::AccountProfiles;

use crate::AppState;
use crate::components::*;
use crate::layouts::application::{ Layout, Main };
use crate::utils::props::*;
use crate::utils::request_graphql::post_graphql;
use crate::variables::*;

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::spawn_local;
use sycamore_router::navigate;
use web_sys::MouseEvent;


//------------------------------------------------------------------------------
/// GraphQL.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct UpdateAccountPublicStatus;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn Detail<G: Html>() -> View<G>
{
    let state: AppState = use_context();
    let selected_account = match state
        .selected_account
        .get_clone()
    {
        Some(selected_account) => selected_account,
        None =>
        {
            navigate("/");
            return view! {};
        },
    };
    let account_name = selected_account.account_name.clone();
    let cloned_account_name = account_name.clone();
    let is_public = create_signal(selected_account.is_public);
    let open_change_public_dialog = create_signal(false);

    let update_account_public_status_handler = move |_|
    {
        let cloned_state = state.clone();
        let cloned_account_name = cloned_account_name.clone();
        spawn_local(async move
        {
            let input = update_account_public_status::UpdateAccountInput
            {
                account_name: cloned_account_name,
                is_public: Some(!is_public.get()),
                email: None,
                default_account_profile_token: None,
                default_workspace_name: None,
            };
            if let Ok(_) = post_graphql::<UpdateAccountPublicStatus>
            (
                &cloned_state,
                "/account/graphql",
                update_account_public_status::Variables
                {
                    input
                },
            ).await
            {
                is_public.set(!is_public.get());
                open_change_public_dialog.set(false);
            };
        });
    };

    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.detail.heading"))
            {
                Box(classes=StrProp("flex flex_column flex_gap_md").into())
                {
                    Heading(variant=HeadingVariant::Bullet.into())
                    {
                        (t!("pages.account.detail.account.heading"))
                    }
                    (t!("pages.account.detail.account.account_name", account_name = selected_account.account_name))
                    Box(classes=StrProp("flex flex_row flex_gap_sm flex_align_center").into())
                    {
                        Switch
                        (
                            checked=is_public,
                            name=StrProp("is_public").into(),
                            on:click=move |event: MouseEvent|
                            {
                                open_change_public_dialog.set(true);
                                event.prevent_default();
                            },
                        )
                        (
                            if is_public.get()
                            {
                                view! { (t!("pages.account.detail.account.public")) }
                            }
                            else
                            {
                                view! { (t!("pages.account.detail.account.private")) }
                            }
                        )
                        Dialog(open=open_change_public_dialog)
                        {
                            DialogHead
                            {
                                (t!("pages.account.detail.account.dialog.change_public.head"))
                            }
                            DialogBody
                            {
                                (
                                    if is_public.get()
                                    {
                                        view! { (t!("pages.account.detail.account.dialog.change_public.body.to_private")) }
                                    }
                                    else
                                    {
                                        view! { (t!("pages.account.detail.account.dialog.change_public.body.to_public")) }
                                    }
                                )
                            }
                            DialogFoot(classes=StrProp("flex flex_gap_md").into())
                            {
                                Button
                                (
                                    variant=ButtonVariant::Outlined.into(),
                                    on:click=move |_|
                                    {
                                        open_change_public_dialog.set(false);
                                    },
                                )
                                {
                                    (t!("pages.account.detail.account.dialog.change_public.button.cancel"))
                                }
                                Button
                                (
                                    variant=ButtonVariant::Filled.into(),
                                    on:click=update_account_public_status_handler,
                                )
                                {
                                    (t!("pages.account.detail.account.dialog.change_public.button.submit"))
                                }
                            }
                        }
                    }
                }
                Box(classes=StrProp("flex flex_column flex_gap_md").into())
                {
                    Heading(variant=HeadingVariant::Bullet.into())
                    {
                        (t!("pages.account.detail.profiles.heading"))
                    }
                    AccountProfiles(account_name=account_name)
                    Button
                    (
                        href=OptionProp(Some("/account/profile/create".to_string())).into(),
                        classes=StrProp("flex_align_self_center").into(),
                        round=ButtonRound::Full.into(),
                        left_icon=view!
                        {
                            Icon
                            (
                                icon=IconKind::Plus.into(),
                                color=Colors::PrimaryText.into(),
                            )
                        },
                    )
                    {
                        (t!("pages.account.detail.profiles.button.create"))
                    }
                }
                Box(classes=StrProp("flex flex_column flex_gap_md").into())
                {
                    Heading(variant=HeadingVariant::Bullet.into())
                    {
                        (t!("pages.account.detail.groups.heading"))
                    }
                    Button
                    (
                        href=OptionProp(Some("/account/group/create".to_string())).into(),
                        classes=StrProp("flex_align_self_center").into(),
                        round=ButtonRound::Full.into(),
                        left_icon=view!
                        {
                            Icon
                            (
                                icon=IconKind::Plus.into(),
                                color=Colors::PrimaryText.into(),
                            )
                        },
                    )
                    {
                        (t!("pages.account.detail.groups.button.create"))
                    }
                }
            }
        }
    }
}
