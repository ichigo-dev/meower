//------------------------------------------------------------------------------
//! Application state.
//------------------------------------------------------------------------------

use crate::Config;
use crate::utils::request_graphql::post_graphql;
use crate::types::SelectedAccount;

use std::str::FromStr;
use std::sync::{ Arc, RwLock };

use chrono::Locale;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use reqwest::header::HeaderMap;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// GraphQL.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/query/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct GetAccountStateDataQuery;


//------------------------------------------------------------------------------
/// Application state.
//------------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct AppState
{
    pub config: Config,
    pub client: Client,
    pub selected_account: Signal<Option<SelectedAccount>>,
    pub datetime_locale: Locale,
    pub access_token: Arc<RwLock<String>>,
}

impl AppState
{
    //--------------------------------------------------------------------------
    /// Creates a new application state.
    //--------------------------------------------------------------------------
    pub async fn new() -> Self
    {
        let config = Config::init();
        let headers = HeaderMap::from_iter(vec!
        [
            (
                config.client_id_key.clone().parse().unwrap(),
                config.client_id.clone().parse().unwrap(),
            ),
        ]);
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or(Client::new());

        let public_user_id = config.public_user_id.clone();
        let datetime_locale = Locale::from_str
            (
                &sys_locale::get_locale()
                    .unwrap_or("en-US".to_string())
                    .replace("-", "_")
            )
            .unwrap_or(Locale::en_US);

        let access_token = Arc::new
        (
            RwLock::new(config.initial_access_token.clone())
        );

        let state = Self
        {
            config,
            client,
            selected_account: create_signal(None),
            datetime_locale,
            access_token,
        };

        // Initializes the account state.
        let data = post_graphql::<GetAccountStateDataQuery>
        (
            &state,
            "/account/graphql",
            get_account_state_data_query::Variables
            {
                public_user_id: public_user_id,
            },
        ).await.unwrap();

        if let Some(account) = data.default_account
        {
            let mut selected_account = SelectedAccount
            {
                account_name: account.account_name.clone(),
                name: "".to_string(),
                avatar_file_key: "".to_string(),
            };

            if let Some(profile) = account.default_account_profile
            {
                selected_account.name = profile.name.clone();

                if let Some(avatar) = profile.avatar
                {
                    selected_account.avatar_file_key = avatar.file_key.clone();
                };
            };
            state.selected_account.set(Some(selected_account));
        };

        state
    }
}
