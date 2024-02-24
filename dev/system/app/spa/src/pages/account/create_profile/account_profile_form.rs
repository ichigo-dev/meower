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
                    s if &s == "male" => Some(create_account::Gender::MALE),
                    s if &s == "female" => Some(create_account::Gender::FEMALE),
                    s if &s == "other" => Some(create_account::Gender::OTHER),
                    _ => None,
                }
            },
            None => None,
        };

        let create_account_profile_input = create_account_profile::CreateAccountProfileInput
        {
            account_name: state.selected_account_name.get_clone(),
            name: values.get("name").unwrap_or("".to_string()),
            affiliation: values.get("affiliation").clone(),
            email: values.get("email").unwrap_or("".to_string()),
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
                 create_account::Variables { create_account_profile_input },
            ).await
            {
                Ok(data) =>
                {
                    let href = format!
                    (
                        "/account/{}",
                        data.create_account_profile.account_name,
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
                label=t!("pages.account_profile.create.form.name.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("name").into(),
                    placeholder=StringProp(t!("pages.account_profile.create.form.name.placeholder")).into(),
                    required=BoolProp(true).into(),
                )
            }
            Label(label=t!("pages.account_profile.create.form.affiliation.label"))
            {
                TextField
                (
                    name=StrProp("affiliation").into(),
                    placeholder=StringProp(t!("pages.account_profile.create.form.affiliation.placeholder")).into(),
                )
            }
            Label
            (
                label=t!("pages.account_profile.create.form.email.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("email").into(),
                    placeholder=StringProp(t!("pages.account_profile.create.form.email.placeholder")).into(),
                    field_type=StrProp("email").into(),
                    required=BoolProp(true).into(),
                )
            }
            Label(label=t!("pages.account_profile.create.form.bio.label"))
            {
                TextField
                (
                    name=StrProp("bio").into(),
                    placeholder=StringProp(t!("pages.account_profile.create.form.bio.placeholder")).into(),
                    multiline=BoolProp(true).into(),
                )
            }
            Label(label=t!("pages.account_profile.create.form.birthdate.label"))
            {
                TextField
                (
                    name=StrProp("birthdate").into(),
                    field_type=StrProp("date").into(),
                )
            }
            Label(label=t!("pages.account_profile.create.form.gender.label"))
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
                                label=t!(&format!("pages.account_profile.create.form.gender.{}.label", gender)),
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
            Button(button_type=StrProp("submit").into())
            {
                (t!("pages.account_profile.create.form.button.send"))
            }
        }
    }
}
