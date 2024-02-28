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
             token: token,
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

    log::info!("account_profile: {:?}", account_profile);

    view!
    {
        Layout
        {
            Main(heading=t!("pages.account.edit_profile.heading"))
            {
                AccountProfileForm
            }
        }
    }
}
