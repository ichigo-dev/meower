//------------------------------------------------------------------------------
//! Account profiles.
//------------------------------------------------------------------------------

use super::account_profile_card::AccountProfileCard;
use crate::AppState;
use crate::utils::request_graphql::post_graphql;

use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use sycamore::prelude::*;
use sycamore::futures::create_resource;


//------------------------------------------------------------------------------
/// Gets a account profile.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/query/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct GetAccountProfiles;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub async fn AccountProfiles<G: Html>( account_name: String ) -> View<G>
{
    let account_profiles = create_signal(Vec::new());
    create_resource(async move
    {
        let mut state: AppState = use_context();
        if let Ok(data) = post_graphql::<GetAccountProfiles>
        (
            &mut state,
            "/account/graphql",
             get_account_profiles::Variables
             {
                 account_name: account_name,
             },
        ).await
        {
            account_profiles.set(data.account_profiles);
        };
    });

    view!
    {
        Indexed
        (
            iterable=*account_profiles,
            view=|profile|
            {
                let account_name = match profile.account
                {
                    Some(account) => account.account_name.clone(),
                    None => "".to_string(),
                };
                let avatar_file_key = match profile.avatar
                {
                    Some(avatar) => avatar.file_key.clone(),
                    None => "".to_string(),
                };
                let cover_file_key = match profile.cover
                {
                    Some(cover) => cover.file_key.clone(),
                    None => "".to_string(),
                };

                view!
                {
                    AccountProfileCard
                    (
                        account_name=account_name,
                        name=profile.name.clone(),
                        avatar_file_key=avatar_file_key,
                        cover_file_key=cover_file_key,
                    )
                }
            }
        )
    }
}
