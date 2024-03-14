//------------------------------------------------------------------------------
//! Group detail page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::layouts::application::{ Layout, Main };
use crate::utils::props::*;
use crate::utils::request_graphql::post_graphql;

use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use sycamore_router::navigate;


//------------------------------------------------------------------------------
/// GraphQL.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/query/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct GetGroupWithMembers;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/query/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct SearchAccounts;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub async fn Detail<G: Html>( group_name: String ) -> View<G>
{
    let state: AppState = use_context();
    if let None = state.selected_account.get_clone()
    {
        navigate("/");
        return view! {};
    };

    let cover_file_key = create_signal(Some(String::new()));
    let avatar_file_key = create_signal(Some(String::new()));
    let edit_href = format!("/account/group/edit/{}", group_name.clone());

    let group = match post_graphql::<GetGroupWithMembers>
    (
        &state,
        "/account/graphql",
         get_group_with_members::Variables
         {
             group_name,
         },
    ).await
    {
        Ok(data) =>
        {
            let cloned_group = data.group.clone();
            if let Some(avatar) = cloned_group.avatar
            {
                avatar_file_key.set(Some(avatar.file_key));
            };
            if let Some(cover) = cloned_group.cover
            {
                cover_file_key.set(Some(cover.file_key));
            };
            data.group
        },
        Err(_) =>
        {
            navigate("/");
            return view! {};
        },
    };
    let members = create_signal(group.group_members);

    // Invite members.
    let open_invite_dialog = create_signal(false);
    let invite_members = create_signal(Vec::new());
    let cloned_state = state.clone();
    let search_handler = move |values: FormValues, _|
    {
        let state = cloned_state.clone();
        let search = values.get("search").unwrap_or("".to_string());

        spawn_local_scoped(async move
        {
            if let Ok(data) = post_graphql::<SearchAccounts>
            (
                &state,
                "/account/graphql",
                 search_accounts::Variables { search },
            ).await
            {
                invite_members.set(data.search_accounts);
                open_invite_dialog.set(true);
            };
        });
    };

    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.group.detail.heading"))
            {
                Box(classes=StrProp("flex flex_justify_between").into())
                {
                    Button
                    (
                        round=ButtonRound::Full.into(),
                        variant=ButtonVariant::Outlined.into(),
                        on:click=move |_|
                        {
                            let window = web_sys::window().unwrap();
                            let history = window.history().unwrap();
                            let _ = history.back();
                        },
                    )
                    {
                        (t!("pages.account.group.detail.button.back"))
                    }
                    Button
                    (
                        round=ButtonRound::Full.into(),
                        href=OptionProp(Some(edit_href)).into(),
                    )
                    {
                        (t!("pages.account.group.detail.button.edit"))
                    }
                }

                GroupCover(file_key=*cover_file_key)
                Box(classes=StrProp("flex flex_row flex_align_center flex_gap_md").into())
                {
                    GroupAvatar
                    (
                        file_key=*avatar_file_key,
                        size=AvatarSize::XXXXXL.into(),
                    )
                    Box
                    {
                        Typography(font_size=TypographyFontSize::XL.into())
                        {
                            (group.name)
                        }
                        Typography(font_weight=TypographyFontWeight::Light.into())
                        {
                            "@" (group.group_name)
                        }
                    }
                }
                Table
                {
                    TableBody
                    {
                        (
                            match group.description.clone()
                            {
                                Some(description) if description.len() > 0 =>
                                {
                                    view!
                                    {
                                        TableRow
                                        {
                                            TableCell
                                            (
                                                is_head=BoolProp(true).into(),
                                                min=BoolProp(true).into(),
                                            )
                                            {
                                                (t!("pages.account.group.detail.table.description"))
                                            }
                                            TableCell
                                            {
                                                (description)
                                            }
                                        }
                                    }
                                },
                                _ => view! {},
                            }
                        )
                        (
                            match group.representative.clone()
                            {
                                Some(representative) if representative.len() > 0 =>
                                {
                                    view!
                                    {
                                        TableRow
                                        {
                                            TableCell
                                            (
                                                is_head=BoolProp(true).into(),
                                                min=BoolProp(true).into(),
                                            )
                                            {
                                                (t!("pages.account.group.detail.table.representative"))
                                            }
                                            TableCell
                                            {
                                                (representative)
                                            }
                                        }
                                    }
                                },
                                _ => view! {},
                            }
                        )
                        (
                            match group.location.clone()
                            {
                                Some(location) if location.len() > 0 =>
                                {
                                    view!
                                    {
                                        TableRow
                                        {
                                            TableCell
                                            (
                                                is_head=BoolProp(true).into(),
                                                min=BoolProp(true).into(),
                                            )
                                            {
                                                (t!("pages.account.group.detail.table.location"))
                                            }
                                            TableCell
                                            {
                                                (location)
                                            }
                                        }
                                    }
                                },
                                _ => view! {},
                            }
                        )
                        (
                            match group.email.clone()
                            {
                                Some(email) if email.len() > 0 =>
                                {
                                    view!
                                    {
                                        TableRow
                                        {
                                            TableCell
                                            (
                                                is_head=BoolProp(true).into(),
                                                min=BoolProp(true).into(),
                                            )
                                            {
                                                (t!("pages.account.group.detail.table.email"))
                                            }
                                            TableCell
                                            {
                                                (email)
                                            }
                                        }
                                    }
                                },
                                _ => view! {},
                            }
                        )
                        (
                            match group.telno.clone()
                            {
                                Some(telno) if telno.len() > 0 =>
                                {
                                    view!
                                    {
                                        TableRow
                                        {
                                            TableCell
                                            (
                                                is_head=BoolProp(true).into(),
                                                min=BoolProp(true).into(),
                                            )
                                            {
                                                (t!("pages.account.group.detail.table.telno"))
                                            }
                                            TableCell
                                            {
                                                (telno)
                                            }
                                        }
                                    }
                                },
                                _ => view! {},
                            }
                        )
                        (
                            match group.founded_at.clone()
                            {
                                Some(founded_at) =>
                                {
                                    view!
                                    {
                                        TableRow
                                        {
                                            TableCell
                                            (
                                                is_head=BoolProp(true).into(),
                                                min=BoolProp(true).into(),
                                            )
                                            {
                                                (t!("pages.account.group.detail.table.founded_at"))
                                            }
                                            TableCell
                                            {
                                                (
                                                    founded_at
                                                        .and_utc()
                                                        .format_localized
                                                        (
                                                            "%x",
                                                            state.datetime_locale,
                                                        )
                                                        .to_string()
                                                )
                                            }
                                        }
                                    }
                                },
                                _ => view! {},
                            }
                        )
                        TableRow
                        {
                            TableCell
                            (
                                is_head=BoolProp(true).into(),
                                min=BoolProp(true).into(),
                            )
                            {
                                (t!("pages.account.group.detail.table.is_public"))
                            }
                            TableCell
                            {
                                (
                                    if group.is_public
                                    {
                                        view!
                                        {
                                            (t!("pages.account.group.detail.table.public"))
                                        }
                                    }
                                    else
                                    {
                                        view!
                                        {
                                            (t!("pages.account.group.detail.table.private"))
                                        }
                                    }
                                )
                            }
                        }
                    }
                }
                Heading(variant=HeadingVariant::Bullet.into())
                {
                    (t!("pages.account.group.detail.members.heading"))
                }
                Box
                {
                    Form
                    (
                        classes=StrProp("flex flex_row flex_align_center flex_gap_md").into(),
                        on_submit=Box::new(search_handler),
                    )
                    {
                        TextField
                        (
                            name=StrProp("search").into(),
                            placeholder=StringProp(t!("pages.account.group.detail.members.invite.placeholder")).into(),
                            full_width=BoolProp(true).into(),
                        )
                        Button
                        (
                            button_type=StrProp("submit").into(),
                            round=ButtonRound::Full.into(),
                        )
                        {
                            (t!("pages.account.group.detail.members.invite.button.search"))
                        }
                    }
                    Dialog(open=open_invite_dialog)
                    {
                        DialogHead
                        {
                            (t!("pages.account.group.detail.members.invite.dialog.head"))
                        }
                        DialogBody
                        {
                            (
                                if invite_members.get_clone().len() > 0
                                {
                                    view!
                                    {
                                        List(variant=ListVariant::Simple.into())
                                        {
                                            Indexed
                                            (
                                                iterable=*invite_members,
                                                view=|member|
                                                {
                                                    let account_name = create_signal(member.account_name);
                                                    let name = create_signal("".to_string());
                                                    let avatar_file_key = create_signal("".to_string());

                                                    if let Some(profile) = member.default_account_profile
                                                    {
                                                        name.set(profile.name);
                                                        if let Some(avatar) = profile.avatar
                                                        {
                                                            avatar_file_key.set(avatar.file_key);
                                                        };
                                                    };

                                                    view!
                                                    {
                                                        ListItem(clickable=BoolProp(true).into())
                                                        {
                                                            MiniProfile
                                                            (
                                                                account_name=account_name.get_clone(),
                                                                name=name.get_clone(),
                                                                file_key=avatar_file_key.get_clone(),
                                                                clickable=BoolProp(true).into(),
                                                            )
                                                        }
                                                    }
                                                },
                                            )
                                        }
                                    }
                                }
                                else
                                {
                                    view!
                                    {
                                        (t!("pages.account.group.detail.members.invite.dialog.not_found"))
                                    }
                                }
                            )
                        }
                        DialogFoot
                        {
                            Button
                            (
                                variant=ButtonVariant::Outlined.into(),
                                on:click=move |_|
                                {
                                    open_invite_dialog.set(false);
                                },
                            )
                            {
                                (t!("pages.account.group.detail.members.invite.dialog.button.cancel"))
                            }
                        }
                    }
                }
                List(variant=ListVariant::Simple.into())
                {
                    Indexed
                    (
                        iterable=*members,
                        view=|member|
                        {
                            let account_name = create_signal("".to_string());
                            let name = create_signal("".to_string());
                            let avatar_file_key = create_signal("".to_string());

                            if let Some(account) = member.account
                            {
                                account_name.set(account.account_name);
                            };
                            if let Some(profile) = member.account_profile
                            {
                                name.set(profile.name);
                                if let Some(avatar) = profile.avatar
                                {
                                    avatar_file_key.set(avatar.file_key);
                                };
                            };

                            view!
                            {
                                ListItem
                                {
                                    MiniProfile
                                    (
                                        account_name=account_name.get_clone(),
                                        name=name.get_clone(),
                                        file_key=avatar_file_key.get_clone(),
                                    )
                                }
                            }
                        },
                    )
                }
            }
        }
    }
}
