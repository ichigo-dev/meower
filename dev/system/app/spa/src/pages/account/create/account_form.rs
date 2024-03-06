//------------------------------------------------------------------------------
//! Account create form.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::types::SelectedAccount;
use crate::utils::request_graphql::post_graphql;
use crate::utils::props::*;

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use sycamore_router::navigate;


//------------------------------------------------------------------------------
/// Creates a new account.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct CreateAccount;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn AccountForm<G: Html>() -> View<G>
{
    let state: AppState = use_context();
    let alert_message = create_signal("".to_string());
    let user_email = state.config.user_email.clone();

    let save_handler = move |values: FormValues, _|
    {
        let state = state.clone();
        let account_name = values
            .get("account_name")
            .unwrap_or("".to_string())
            .to_string();
        let email = values
            .get("email")
            .unwrap_or("".to_string())
            .to_string();
        let is_public = values
            .get("is_public")
            .is_some();

        let create_account_input = create_account::CreateAccountInput
        {
            public_user_id: state.config.public_user_id.clone(),
            account_name: account_name.clone(),
            email: email.clone(),
            is_public: is_public,
        };

        spawn_local_scoped(async move
        {
            match post_graphql::<CreateAccount>
            (
                &state,
                "/account/graphql",
                 create_account::Variables { create_account_input },
            ).await
            {
                Ok(data) =>
                {
                    let account = data.create_account;
                    let mut selected_account = SelectedAccount
                    {
                        account_name: account.account_name.clone(),
                        name: "".to_string(),
                        avatar_file_key: "".to_string(),
                    };

                    if let Some(profile) = account.default_account_profile
                    {
                        selected_account.name = profile.name;

                        if let Some(avatar) = profile.avatar
                        {
                            selected_account.avatar_file_key = avatar.file_key;
                        }
                    }

                    state.selected_account.set
                    (
                        Some(selected_account)
                    );

                    let href = format!
                    (
                        "/account/{}",
                        account.account_name,
                    );
                    navigate(&href);
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
        Form
        (
            classes=StrProp("flex flex_column flex_gap_md width_full").into(),
            on_submit=Box::new(save_handler),
        )
        {
            Label
            (
                label=t!("pages.account.create.form.account_name.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("account_name").into(),
                    placeholder=StringProp(t!("pages.account.create.form.account_name.placeholder")).into(),
                    required=BoolProp(true).into(),
                )
            }
            Label
            (
                label=t!("pages.account.create.form.email.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("email").into(),
                    placeholder=StringProp(t!("pages.account.create.form.email.placeholder")).into(),
                    field_type=StrProp("email").into(),
                    required=BoolProp(true).into(),
                    value=StringProp(user_email).into(),
                )
            }
            Label
            (
                label=t!("pages.account.create.form.is_public.label"),
                required=BoolProp(true).into(),
            )
            {
                Checkbox(name=StrProp("is_public").into())
            }
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
            Box(classes=StrProp("flex flex_row flex_justify_between width_full").into())
            {
                Button
                (
                    button_type=StrProp("button").into(),
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
                    (t!("pages.account.create.form.button.cancel"))
                }
                Button
                (
                    button_type=StrProp("submit").into(),
                    classes=StrProp("flex_align_self_end").into(),
                    round=ButtonRound::Full.into(),
                )
                {
                    (t!("pages.account.create.form.button.send"))
                }
            }
        }
    }
}
