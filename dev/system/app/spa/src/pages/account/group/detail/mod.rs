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
struct GetGroup;


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

    let group = match post_graphql::<GetGroup>
    (
        &state,
        "/account/graphql",
         get_group::Variables
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
            }
        }
    }
}
