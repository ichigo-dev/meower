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
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{ Event, FileReader, HtmlInputElement, MouseEvent, ProgressEvent };


//------------------------------------------------------------------------------
/// GraphQL.
//------------------------------------------------------------------------------
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
struct CreateAccountProfileAdditional;

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
    let upload_avatar_file_name = create_signal(None);
    let upload_avatar_base64 = create_signal(None);
    let delete_avatar_file = create_signal(false);
    let cover_base64 = create_signal(None);
    let upload_cover_file_name = create_signal(None);
    let upload_cover_base64 = create_signal(None);
    let delete_cover_file = create_signal(false);

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
                token: token.clone(),
                name: values.get("name").unwrap_or("".to_string()),
                affiliation: values.get("affiliation").clone(),
                location: values.get("location").clone(),
                email: values.get("email").clone(),
                telno: values.get("telno").clone(),
                bio: values.get("bio").clone(),
                birthdate: birthdate,
                gender: gender,
            };

            let upload_avatar_input = update_account_profile::UploadAvatarInput
            {
                account_profile_token: token.clone(),
                file_name: upload_avatar_file_name.get_clone(),
                base64: upload_avatar_base64.get_clone(),
                delete_file: delete_avatar_file.get_clone(),
            };

            let upload_cover_input = update_account_profile::UploadCoverInput
            {
                account_profile_token: token.clone(),
                file_name: upload_cover_file_name.get_clone(),
                base64: upload_cover_base64.get_clone(),
                delete_file: delete_cover_file.get_clone(),
            };

            spawn_local_scoped(async move
            {
                match post_graphql::<UpdateAccountProfile>
                (
                    &mut state,
                    "/account/graphql",
                     update_account_profile::Variables
                     {
                         update_account_profile_input,
                         upload_avatar_input,
                         upload_cover_input,
                     },
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
                match post_graphql::<CreateAccountProfile>
                (
                    &mut state,
                    "/account/graphql",
                     create_account_profile::Variables
                     {
                         create_account_profile_input,
                     },
                ).await
                {
                    Ok(data) =>
                    {
                        let upload_avatar_input = create_account_profile_additional::UploadAvatarInput
                        {
                            account_profile_token: data.create_account_profile.token.clone(),
                            file_name: upload_avatar_file_name.get_clone(),
                            base64: upload_avatar_base64.get_clone(),
                            delete_file: false,
                        };

                        let upload_cover_input = create_account_profile_additional::UploadCoverInput
                        {
                            account_profile_token: data.create_account_profile.token,
                            file_name: upload_cover_file_name.get_clone(),
                            base64: upload_cover_base64.get_clone(),
                            delete_file: false,
                        };

                        post_graphql::<CreateAccountProfileAdditional>
                        (
                            &mut state,
                            "/account/graphql",
                            create_account_profile_additional::Variables
                            {
                                upload_avatar_input,
                                upload_cover_input,
                            },
                        ).await.unwrap();

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

    let avatar_file_key = create_signal(Some(props.avatar_file_key.clone()));
    let cover_file_key = create_signal(Some(props.cover_file_key.clone()));

    view!
    {
        Form
        (
            classes=StrProp("flex flex_column flex_gap_md width_full").into(),
            on_submit=Box::new(save_handler),
        )
        {
            Label(label=t!("pages.account.components.account_profile.form.cover.label"))
            {
                ProfileCover
                (
                    file_key=*cover_file_key,
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
                            upload_cover_base64.set(Some(base64.clone()));
                            upload_cover_file_name.set(Some(cloned_file.name()));
                        }) as Box<dyn Fn(ProgressEvent)>);
                        reader.set_onload(Some(closure.as_ref().unchecked_ref()));
                        reader.read_as_data_url(&file).unwrap();
                        closure.forget();
                    },
                )
                (
                    if cover_file_key.get_clone().unwrap_or_default().len() > 0
                        || upload_cover_file_name.get_clone().is_some()
                    {
                        view!
                        {
                            Button
                            (
                                attr:style="align-self: flex-start;",
                                on:click=move |event: MouseEvent|
                                {
                                    delete_cover_file.set(true);
                                    upload_cover_base64.set(None);
                                    upload_cover_file_name.set(None);
                                    cover_base64.set(None);
                                    cover_file_key.set(Some("".to_string()));
                                    event.prevent_default();
                                },
                            )
                            {
                                (t!("pages.account.components.account_profile.form.cover.button.remove"))
                            }
                        }
                    }
                    else
                    {
                        view! {}
                    }
                )
            }

            Label
            (
                label=t!("pages.account.components.account_profile.form.avatar.label"),
                attr:style="align-self: flex-start; width: auto;",
            )
            {
                ProfileAvatar
                (
                    file_key=*avatar_file_key,
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
                            upload_avatar_base64.set(Some(base64.clone()));
                            upload_avatar_file_name.set(Some(cloned_file.name()));
                        }) as Box<dyn Fn(ProgressEvent)>);
                        reader.set_onload(Some(closure.as_ref().unchecked_ref()));
                        reader.read_as_data_url(&file).unwrap();
                        closure.forget();
                    },
                )
                (
                    if avatar_file_key.get_clone().unwrap_or_default().len() > 0
                        || upload_avatar_file_name.get_clone().is_some()
                    {
                        view!
                        {
                            Button
                            (
                                attr:style="align-self: flex-start;",
                                on:click=move |event: MouseEvent|
                                {
                                    delete_avatar_file.set(true);
                                    upload_avatar_base64.set(None);
                                    upload_avatar_file_name.set(None);
                                    avatar_base64.set(None);
                                    avatar_file_key.set(Some("".to_string()));
                                    event.prevent_default();
                                },
                            )
                            {
                                (t!("pages.account.components.account_profile.form.avatar.button.remove"))
                            }
                        }
                    }
                    else
                    {
                        view! {}
                    }
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
                    (t!("pages.account.components.account_profile.form.button.cancel"))
                }
                Button
                (
                    button_type=StrProp("submit").into(),
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
}
