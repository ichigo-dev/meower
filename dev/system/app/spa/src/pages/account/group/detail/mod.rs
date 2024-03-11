//------------------------------------------------------------------------------
//! Group detail page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::layouts::application::{ Layout, Main };
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

    let cloned_group_name = group_name.clone();
    match post_graphql::<GetGroup>
    (
        &state,
        "/account/graphql",
         get_group::Variables
         {
             group_name: cloned_group_name,
         },
    ).await
    {
        Ok(data) =>
        {
            let cloned_group = data.group.clone();
            log::info!("{:?}", cloned_group);
            if let Some(avatar) = cloned_group.avatar
            {
                avatar_file_key.set(Some(avatar.file_key));
            };
            if let Some(cover) = cloned_group.cover
            {
                cover_file_key.set(Some(cover.file_key));
            };
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
                GroupCover(file_key=*cover_file_key)
                GroupAvatar
                (
                    file_key=*avatar_file_key,
                    size=AvatarSize::XXXXXL.into(),
                )
            }
        }
    }
}
