//------------------------------------------------------------------------------
//! Create profile form.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::request_graphql::post_graphql;
use crate::utils::props::*;

use chrono::{ NaiveDate, NaiveDateTime };
use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use sycamore_router::navigate;


//------------------------------------------------------------------------------
/// Creates a new account profile.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct CreateAccountProfile;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn AccountProfileForm<G: Html>() -> View<G>
{
    let state: AppState = use_context();
    let alert_message = create_signal("".to_string());

    let save_handler = move |values: FormValues, _|
    {
        let mut state = state.clone();

        let birthdate = match values.get("birthdate")
        {
            Some(birthdate) =>
            {
                match NaiveDate::parse_from_str
                (
                    &birthdate,
                    "%Y-%m-%d",
                )
                {
                    Ok(naive_date) => Some(NaiveDateTime::from(naive_date)),
                    Err(_) => None,
                }
            },
            None => None,
        };

        let gender = match values.get("gender")
        {
            Some(gender) =>
            {
                match gender
                {
                    s if &s == "male" =>
                    {
                        Some(create_account_profile::Gender::MALE)
                    },
                    s if &s == "female" =>
                    {
                        Some(create_account_profile::Gender::FEMALE)
                    },
                    s if &s == "other" =>
                    {
                        Some(create_account_profile::Gender::OTHER)
                    },
                    _ => None,
                }
            },
            None => None,
        };

        let selected_account = match state.selected_account.get_clone()
        {
            Some(selected_account) => selected_account,
            None =>
            {
                alert_message.set(t!("pages.account.components.account_profile.form.error.account_not_selected"));
                return;
            },
        };
        let create_account_profile_input
            = create_account_profile::CreateAccountProfileInput
        {
            account_name: selected_account.account_name.clone(),
            name: values.get("name").unwrap_or("".to_string()),
            affiliation: values.get("affiliation").clone(),
            location: values.get("location").clone(),
            email: values.get("email").clone(),
            telno: values.get("telno").clone(),
            bio: values.get("bio").clone(),
            birthdate: birthdate,
            gender: gender,
        };

        spawn_local_scoped(async move
        {
            match post_graphql::<CreateAccountProfile>
            (
                &mut state,
                "/account/graphql",
                 create_account_profile::Variables
                 {
                     create_account_profile_input
                 },
            ).await
            {
                Ok(data) =>
                {
                    let href = match data.create_account_profile.account
                    {
                        Some(account) =>
                        {
                            format!("/account/{}", account.account_name)
                        },
                        None => "/".to_string(),
                    };
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

    let genders = create_signal(vec!
    [
        "male".to_string(),
        "female".to_string(),
        "other".to_string(),
    ]);

    view!
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
            Label
            (
                label=t!("pages.account.components.account_profile.form.name.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("name").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.name.placeholder")).into(),
                    required=BoolProp(true).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.affiliation.label"))
            {
                TextField
                (
                    name=StrProp("affiliation").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.affiliation.placeholder")).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.location.label"))
            {
                TextField
                (
                    name=StrProp("location").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.location.placeholder")).into(),
                )
            }
            Label
            (
                label=t!("pages.account.components.account_profile.form.email.label"),
            )
            {
                TextField
                (
                    name=StrProp("email").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.email.placeholder")).into(),
                    field_type=StrProp("email").into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.telno.label"))
            {
                TextField
                (
                    name=StrProp("telno").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.telno.placeholder")).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.bio.label"))
            {
                TextField
                (
                    name=StrProp("bio").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.bio.placeholder")).into(),
                    multiline=BoolProp(true).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.birthdate.label"))
            {
                TextField
                (
                    name=StrProp("birthdate").into(),
                    field_type=StrProp("date").into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.gender.label"))
            {
                RadioGroup
                (
                    classes=StrProp("flex flex_row flex_align_center").into(),
                    name=StrProp("gender").into(),
                )
                {
                    Indexed
                    (
                        iterable=*genders,
                        view=|gender| view!
                        {
                            Label
                            (
                                classes=StrProp("width_6xl flex_align_center").into(),
                                direction=LabelDirection::Row.into(),
                                label=t!(&format!("common.constants.gender.{}", gender)),
                            )
                            {
                                Radio
                                (
                                    name=StrProp("gender").into(),
                                    checked=BoolProp(&gender == "other").into(),
                                    value=*create_signal(gender),
                                )
                            }
                        },
                    )
                }
            }
            Button
            (
                button_type=StrProp("submit").into(),
                classes=StrProp("flex_align_self_end").into(),
                round=ButtonRound::Full.into(),
            )
            {
                (t!("pages.account.components.account_profile.form.button.send"))
            }
        }
    }
}
