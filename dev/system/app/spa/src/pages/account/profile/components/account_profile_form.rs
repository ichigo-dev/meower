//------------------------------------------------------------------------------
//! Create profile form.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::types::UploadFile;
use crate::utils::request_graphql::post_graphql_with_files;
use crate::utils::props::*;

use base64::prelude::*;
use chrono::{ NaiveDate, NaiveDateTime };
use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use sycamore_router::navigate;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{ Event, FileReader, HtmlInputElement, ProgressEvent };


//------------------------------------------------------------------------------
/// GraphQL.
//------------------------------------------------------------------------------
type Upload = ();

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct CreateAccountProfile;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct UpdateAccountProfile;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct AccountProfileFormProps
{
    pub token: Option<String>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub affiliation: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub telno: Option<String>,
    pub birthdate: Option<String>,
    #[prop(default = "other".to_string())]
    pub gender: String,
    #[prop(default)]
    pub avatar_file_key: String,
    #[prop(default)]
    pub cover_file_key: String,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn AccountProfileForm<G: Html>( props: AccountProfileFormProps ) -> View<G>
{
    let state: AppState = use_context();
    let alert_message = create_signal("".to_string());
    let token = props.token.clone();

    let avatar_base64 = create_signal(None);
    let avatar_file = create_signal(None);
    let cover_base64 = create_signal(None);
    let cover_file = create_signal(None);

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

        let mut files = Vec::new();
        if let Some(avatar_file) = avatar_file.get_clone()
        {
            files.push(avatar_file);
        };
        if let Some(cover_file) = cover_file.get_clone()
        {
            files.push(cover_file);
        };

        if let Some(token) = token.clone()
        {
            let gender = match values.get("gender")
            {
                Some(gender) =>
                {
                    match gender
                    {
                        s if &s == "male" =>
                        {
                            Some(update_account_profile::Gender::MALE)
                        },
                        s if &s == "female" =>
                        {
                            Some(update_account_profile::Gender::FEMALE)
                        },
                        s if &s == "other" =>
                        {
                            Some(update_account_profile::Gender::OTHER)
                        },
                        _ => None,
                    }
                },
                None => None,
            };

            let update_account_profile_input
                = update_account_profile::UpdateAccountProfileInput
            {
                token: token,
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
                match post_graphql_with_files::<UpdateAccountProfile>
                (
                    &mut state,
                    "/account/graphql",
                     update_account_profile::Variables
                     {
                         update_account_profile_input,
                         avatar_file: Some(()),
                         cover_file: Some(()),
                     },
                     files,
                ).await
                {
                    Ok(data) =>
                    {
                        let href = match data.update_account_profile.account
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
        }
        else
        {
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
                match post_graphql_with_files::<CreateAccountProfile>
                (
                    &mut state,
                    "/account/graphql",
                     create_account_profile::Variables
                     {
                         create_account_profile_input,
                         avatar_file: Some(()),
                         cover_file: Some(()),
                     },
                     files,
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
            Label(label=t!("pages.account.components.account_profile.form.cover.label"))
            {
                ProfileCover
                (
                    file_key=OptionProp(Some(props.cover_file_key)).into(),
                    base64=cover_base64,
                )
                input
                (
                    class="hide",
                    type="file",
                    accept="image/*",
                    on:change=move |event: Event|
                    {
                        let file = match event
                            .target()
                            .unwrap()
                            .dyn_into::<HtmlInputElement>()
                            .unwrap()
                            .files()
                        {
                            Some(files) =>
                            {
                                match files.get(0)
                                {
                                    Some(file) => file,
                                    None => return,
                                }
                            }
                            None => return,
                        };

                        let reader = FileReader::new().unwrap();
                        let cloned_reader = reader.clone();
                        let cloned_file = file.clone();
                        let closure = Closure::wrap(Box::new(move |_|
                        {
                            let base64 = cloned_reader
                                .result()
                                .unwrap()
                                .as_string()
                                .unwrap();
                            cover_base64.set(Some(base64.clone()));

                            let base64 = base64
                                .split(",")
                                .collect::<Vec<&str>>()[1];
                            let buffer = BASE64_STANDARD
                                .decode(base64.as_bytes())
                                .unwrap();
                            let upload_file = UploadFile
                            {
                                name: "cover_file".to_string(),
                                content: buffer,
                                file_name: cloned_file.name(),
                                mime: cloned_file.type_(),
                                headers: Default::default(),
                            };
                            cover_file.set(Some(upload_file));
                        }) as Box<dyn Fn(ProgressEvent)>);
                        reader.set_onload(Some(closure.as_ref().unchecked_ref()));
                        reader.read_as_data_url(&file).unwrap();
                        closure.forget();
                    },
                )
            }

            Label(label=t!("pages.account.components.account_profile.form.avatar.label"))
            {
                ProfileAvatar
                (
                    file_key=OptionProp(Some(props.avatar_file_key)).into(),
                    size=AvatarSize::XXXXXL.into(),
                    base64=avatar_base64,
                )
                input
                (
                    class="hide",
                    type="file",
                    accept="image/*",
                    on:change=move |event: Event|
                    {
                        let file = match event
                            .target()
                            .unwrap()
                            .dyn_into::<HtmlInputElement>()
                            .unwrap()
                            .files()
                        {
                            Some(files) =>
                            {
                                match files.get(0)
                                {
                                    Some(file) => file,
                                    None => return,
                                }
                            }
                            None => return,
                        };

                        let reader = FileReader::new().unwrap();
                        let cloned_reader = reader.clone();
                        let cloned_file = file.clone();
                        let closure = Closure::wrap(Box::new(move |_|
                        {
                            let base64 = cloned_reader
                                .result()
                                .unwrap()
                                .as_string()
                                .unwrap();
                            avatar_base64.set(Some(base64.clone()));

                            let base64 = base64
                                .split(",")
                                .collect::<Vec<&str>>()[1];
                            let buffer = BASE64_STANDARD
                                .decode(base64.as_bytes())
                                .unwrap();
                            let upload_file = UploadFile
                            {
                                name: "cover_file".to_string(),
                                content: buffer,
                                file_name: cloned_file.name(),
                                mime: cloned_file.type_(),
                                headers: Default::default(),
                            };
                            avatar_file.set(Some(upload_file));
                        }) as Box<dyn Fn(ProgressEvent)>);
                        reader.set_onload(Some(closure.as_ref().unchecked_ref()));
                        reader.read_as_data_url(&file).unwrap();
                        closure.forget();
                    },
                )
            }
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
                    value=StringProp(props.name.unwrap_or_default()).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.affiliation.label"))
            {
                TextField
                (
                    name=StrProp("affiliation").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.affiliation.placeholder")).into(),
                    value=StringProp(props.affiliation.unwrap_or_default()).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.location.label"))
            {
                TextField
                (
                    name=StrProp("location").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.location.placeholder")).into(),
                    value=StringProp(props.location.unwrap_or_default()).into(),
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
                    value=StringProp(props.email.unwrap_or_default()).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.telno.label"))
            {
                TextField
                (
                    name=StrProp("telno").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.telno.placeholder")).into(),
                    value=StringProp(props.telno.unwrap_or_default()).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.bio.label"))
            {
                TextField
                (
                    name=StrProp("bio").into(),
                    placeholder=StringProp(t!("pages.account.components.account_profile.form.bio.placeholder")).into(),
                    multiline=BoolProp(true).into(),
                    value=StringProp(props.bio.unwrap_or_default()).into(),
                )
            }
            Label(label=t!("pages.account.components.account_profile.form.birthdate.label"))
            {
                TextField
                (
                    name=StrProp("birthdate").into(),
                    field_type=StrProp("date").into(),
                    value=StringProp(props.birthdate.unwrap_or_default()).into(),
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
                        view=move |gender|
                        {
                            let default_gender = props.gender.clone();
                            view!
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
                                        checked=BoolProp(gender == default_gender).into(),
                                        value=*create_signal(gender),
                                    )
                                }
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
                (
                    if props.token.is_some()
                    {
                        t!("pages.account.components.account_profile.form.button.update")
                    }
                    else
                    {
                        t!("pages.account.components.account_profile.form.button.send")
                    }
                )
            }
        }
    }
}
