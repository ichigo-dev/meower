//------------------------------------------------------------------------------
//! Account edit page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::layouts::application::{ Layout, Main };
use crate::utils::request_graphql::post_graphql;
use super::components::AccountProfileForm;

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
struct GetEditAccountProfilePageDataQuery;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub async fn Edit<G: Html>( token: String ) -> View<G>
{
    let mut state: AppState = use_context();
    let account_profile = match post_graphql::<GetEditAccountProfilePageDataQuery>
    (
        &mut state,
        "/account/graphql",
         get_edit_account_profile_page_data_query::Variables
         {
             token: token.clone(),
         },
    ).await
    {
        Ok(data) => data.account_profile,
        Err(_) =>
        {
            navigate("/");
            return view! {};
        },
    };

    let birthdate = match account_profile.birthdate
    {
        Some(birthdate) => birthdate.format("%Y-%m-%d").to_string(),
        None => "".to_string(),
    };
    let gender = match account_profile.gender
    {
        Some(gender) =>
        {
            match gender
            {
                get_edit_account_profile_page_data_query::Gender::MALE =>
                {
                    "male".to_string()
                },
                get_edit_account_profile_page_data_query::Gender::FEMALE =>
                {
                    "female".to_string()
                },
                _ => "other".to_string(),
            }
        },
        None => "other".to_string(),
    };

    let avatar_file_key = match account_profile.avatar
    {
        Some(avatar) => avatar.file_key,
        None => "".to_string(),
    };
    let cover_file_key = match account_profile.cover
    {
        Some(cover) => cover.file_key,
        None => "".to_string(),
    };

    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.edit_profile.heading"))
            {
                AccountProfileForm
                (
                    token=token,
                    name=account_profile.name,
                    bio=account_profile.bio.unwrap_or_default(),
                    affiliation=account_profile.affiliation.unwrap_or_default(),
                    location=account_profile.location.unwrap_or_default(),
                    email=account_profile.email.unwrap_or_default(),
                    telno=account_profile.telno.unwrap_or_default(),
                    birthdate=birthdate,
                    gender=gender,
                    avatar_file_key=avatar_file_key,
                    cover_file_key=cover_file_key,
                )
            }
        }
    }
}
