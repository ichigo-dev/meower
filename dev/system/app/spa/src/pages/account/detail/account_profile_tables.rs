//------------------------------------------------------------------------------
//! Account profile table.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::request_graphql::post_graphql;
use crate::utils::props::*;

use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Gets a account profile.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/pages/account/detail.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct GetAccountProfiles;


//------------------------------------------------------------------------------
/// AccountProfileTables.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub async fn AccountProfileTables<G: Html>( account_name: String ) -> View<G>
{
    let mut state: AppState = use_context();
    let data = match post_graphql::<GetAccountProfiles>
    (
        &mut state,
        "/account/graphql",
         get_account_profiles::Variables
         {
             account_name: account_name,
         },
    ).await
    {
        Ok(data) => data,
        Err(e) => return view! { Alert { (e) } },
    };
    let account_profiles = create_signal(data.account_profiles);
    log::info!("account_profiles: {:?}", account_profiles);

    view!
    {
        Indexed
        (
            iterable=*account_profiles,
            view=|account_profile| view!
            {
                Table
                {
                    TableBody
                    {
                        TableRow
                        {
                            TableCell(is_head=BoolProp(true).into())
                            {
                                (t!("pages.account.detail.table.name.head"))
                            }
                            TableCell { (account_profile.name) }
                        }
                        TableRow
                        {
                            TableCell(is_head=BoolProp(true).into())
                            {
                                (t!("pages.account.detail.table.affiliation.head"))
                            }
                            TableCell { (account_profile.affiliation) }
                        }
                        TableRow
                        {
                            TableCell(is_head=BoolProp(true).into())
                            {
                                (t!("pages.account.detail.table.email.head"))
                            }
                            TableCell { (account_profile.email) }
                        }
                        TableRow
                        {
                            TableCell(is_head=BoolProp(true).into())
                            {
                                (t!("pages.account.detail.table.bio.head"))
                            }
                            TableCell
                            {
                                (account_profile.bio)
                            }
                        }
                        TableRow
                        {
                            TableCell(is_head=BoolProp(true).into())
                            {
                                (t!("pages.account.detail.table.birthdate.head"))
                            }
                            TableCell
                            {
                                (
                                    match account_profile.birthdate
                                    {
                                        Some(birthdate) =>
                                        {
                                            birthdate
                                                .format("%Y-%m-%d")
                                                .to_string()
                                        },
                                        None => "".to_string(),
                                    }
                                )
                            }
                        }
                        TableRow
                        {
                            TableCell(is_head=BoolProp(true).into())
                            {
                                (t!("pages.account.detail.table.gender.head"))
                            }
                            TableCell
                            {
                                (
                                    match &account_profile.gender
                                    {
                                        Some(gender) => match gender
                                        {
                                            get_account_profiles::Gender::MALE =>
                                            {
                                                t!("pages.account.detail.table.gender.male")
                                            },
                                            get_account_profiles::Gender::FEMALE =>
                                            {
                                                t!("pages.account.detail.table.gender.female")
                                            },
                                            get_account_profiles::Gender::OTHER =>
                                            {
                                                t!("pages.account.detail.table.gender.other")
                                            },
                                            _ =>
                                            {
                                                "".to_string()
                                            },
                                        },
                                        None => "".to_string(),
                                    }
                                )
                            }
                        }
                    }
                }
            }
        )
    }
}
