//------------------------------------------------------------------------------
//! Account create page.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::layouts::application::Main;
use crate::utils::request_graphql::post_graphql;
use crate::utils::props::*;

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;


//------------------------------------------------------------------------------
/// Creates a new account.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/pages/account/create.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct CreateAccount;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn Create<G: Html>() -> View<G>
{
    let state: AppState = use_context();
    let alert_message = create_signal("".to_string());

    let save_handler = move |values: FormValues, _|
    {
        let state = state.clone();
        let account_name = values
            .get("account_name")
            .unwrap_or(&"".to_string())
            .to_string();
        let input = create_account::CreateAccountInput
        {
            public_user_id: state.config.public_user_id.clone(),
            account_name: account_name.clone(),
        };

        spawn_local_scoped(async move
        {
            match post_graphql::<CreateAccount>
            (
                &state,
                "/account/graphql",
                 create_account::Variables
                 {
                     input: input,
                 },
            ).await
            {
                Ok(data) =>
                {
                    let window = web_sys::window().unwrap();
                    let href = format!
                    (
                        "/account/{}",
                        data.create_account.account_name,
                    );
                    window
                        .location()
                        .set_href(&href).unwrap();
                },
                Err(e) => 
                {
                    alert_message.set(e);
                    return;
                },
            };
        });
    };

    view!
    {
        Main(heading=t!("pages.account.create.heading"))
        {
            Form
            (
                classes=StrProp("flex flex_column flex_gap_md width_full").into(),
                on_submit=Box::new(save_handler),
            )
            {
                (
                    if alert_message.get_clone().len() > 0
                    {
                        view! { Alert { (alert_message.get_clone()) } }
                    }
                    else
                    {
                        view! {}
                    }
                )
                TextField(name=StrProp("account_name").into())
                Button(button_type=StrProp("submit").into())
                {
                    "Send"
                }
            }
        }
    }
}
