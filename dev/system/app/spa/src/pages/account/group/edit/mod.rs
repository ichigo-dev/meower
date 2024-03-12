//------------------------------------------------------------------------------
//! Group edit page.
//------------------------------------------------------------------------------

use super::components::GroupForm;

use crate::AppState;
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
pub async fn Edit<G: Html>( group_name: String ) -> View<G>
{
    let state: AppState = use_context();
    if let None = state.selected_account.get_clone()
    {
        navigate("/");
        return view! {};
    };

    let cover_file_key = create_signal(String::new());
    let avatar_file_key = create_signal(String::new());
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
                avatar_file_key.set(avatar.file_key);
            };
            if let Some(cover) = cloned_group.cover
            {
                cover_file_key.set(cover.file_key);
            };
            data.group
        },
        Err(_) =>
        {
            navigate("/");
            return view! {};
        },
    };

    let founded_at = match group.founded_at
    {
        Some(founded_at) => founded_at.format("%Y-%m-%d").to_string(),
        None => "".to_string(),
    };

    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.group.edit.heading"))
            {
                GroupForm
                (
                    group_name=group.group_name,
                    name=group.name,
                    description=group.description.unwrap_or_default(),
                    representative=group.representative.unwrap_or_default(),
                    location=group.location.unwrap_or_default(),
                    email=group.email.unwrap_or_default(),
                    telno=group.telno.unwrap_or_default(),
                    founded_at=founded_at,
                    is_public=group.is_public,
                    avatar_file_key=avatar_file_key.get_clone(),
                    cover_file_key=cover_file_key.get_clone(),
                )
            }
        }
    }
}
