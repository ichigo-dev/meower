//------------------------------------------------------------------------------
//! Groups.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
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
struct GetGroups;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub async fn Groups<G: Html>( account_name: String ) -> View<G>
{
    let groups = create_signal(Vec::new());

    create_effect(move ||
    {
        let state: AppState = use_context();
        let cloned_account_name = account_name.clone();
        spawn_local_scoped(async move
        {
            if let Ok(data) = post_graphql::<GetGroups>
            (
                &state,
                "/account/graphql",
                 get_groups::Variables
                 {
                     account_name: cloned_account_name,
                 },
            ).await
            {
                groups.set(data.groups);
            };
        });
    });

    view!
    {
        (
            if groups.get_clone().len() > 0
            {
                view! {}
            }
            else
            {
                view!
                {
                    Typography(classes=StrProp("width_full text_align_center").into())
                    {
                        (t!("pages.account.detail.groups.no_group"))
                    }
                }
            }
        )
        List(variant=ListVariant::Simple.into())
        {
            Indexed
            (
                iterable=*groups,
                view=move |group|
                {
                    let group_file_key = match group.avatar
                    {
                        Some(avatar) => avatar.file_key.clone(),
                        None => "".to_string(),
                    };
                    let cloned_group_name = group.group_name.clone();

                    view!
                    {
                        ListItem
                        (
                            classes=StrProp("flex flex_row flex_align_center flex_gap_md").into(),
                            clickable=BoolProp(true).into(),
                            on:click=move |_|
                            {
                                navigate
                                (
                                    &format!
                                    (
                                        "/account/group/{}",
                                        cloned_group_name
                                    )
                                );
                            },
                        )
                        {
                            GroupAvatar
                            (
                                file_key=OptionProp(Some(group_file_key)).into(),
                                size=AvatarSize::XXXXL.into(),
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
            )
        }
    }
}
