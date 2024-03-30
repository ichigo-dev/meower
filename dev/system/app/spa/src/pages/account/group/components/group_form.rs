//------------------------------------------------------------------------------
//! Create group form.
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
struct CreateGroup;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct CreateGroupAdditional;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct UpdateGroup;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct GroupFormProps
{
    pub group_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub representative: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub telno: Option<String>,
    pub founded_at: Option<String>,
    pub is_public: Option<bool>,
    #[prop(default)]
    pub avatar_file_key: String,
    #[prop(default)]
    pub cover_file_key: String,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn GroupForm<G: Html>( props: GroupFormProps ) -> View<G>
{
    let state: AppState = use_context();
    let alert_message = create_signal("".to_string());
    let group_name = props.group_name.clone();
    let cloned_group_name = group_name.clone();

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
        let state = state.clone();

        let founded_at = match values.get("founded_at")
        {
            Some(founded_at) =>
            {
                match NaiveDate::parse_from_str
                (
                    &founded_at,
                    "%Y-%m-%d",
                )
                {
                    Ok(naive_date) => Some(NaiveDateTime::from(naive_date)),
                    Err(_) => None,
                }
            },
            None => None,
        };

        let selected_account = match state.selected_account.get_clone()
        {
            Some(selected_account) => selected_account,
            None =>
            {
                alert_message.set(t!("pages.account.profile.components.account_profile_form.error.account_not_selected"));
                return;
            },
        };
        if let Some(group_name) = cloned_group_name.clone()
        {
            let update_group_input = update_group::UpdateGroupInput
            {
                account_name: selected_account.account_name.clone(),
                group_name: group_name.clone(),
                name: values.get("name").unwrap_or("".to_string()),
                description: values.get("description").clone(),
                representative: values.get("representative").clone(),
                location: values.get("location").clone(),
                email: values.get("email").clone(),
                telno: values.get("telno").clone(),
                founded_at: founded_at,
                is_public: values.get("is_public").is_some(),
            };

            let upload_group_avatar_input = update_group::UploadGroupAvatarInput
            {
                account_name: selected_account.account_name.clone(),
                group_name: group_name.clone(),
                file_name: upload_avatar_file_name.get_clone(),
                base64: upload_avatar_base64.get_clone(),
                delete_file: delete_avatar_file.get_clone(),
            };

            let upload_group_cover_input = update_group::UploadGroupCoverInput
            {
                account_name: selected_account.account_name.clone(),
                group_name: group_name.clone(),
                file_name: upload_cover_file_name.get_clone(),
                base64: upload_cover_base64.get_clone(),
                delete_file: delete_cover_file.get_clone(),
            };

            spawn_local_scoped(async move
            {
                match post_graphql::<UpdateGroup>
                (
                    &state,
                    "/account/graphql",
                     update_group::Variables
                     {
                         update_group_input,
                         upload_group_avatar_input,
                         upload_group_cover_input,
                     },
                ).await
                {
                    Ok(data) =>
                    {
                        let href = format!
                        (
                            "/account/group/{}",
                            data.update_group.group_name
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
        }
        else
        {
            let create_group_input = create_group::CreateGroupInput
            {
                account_name: selected_account.account_name.clone(),
                group_name: values.get("group_name").unwrap_or("".to_string()),
                name: values.get("name").unwrap_or("".to_string()),
                description: values.get("description").clone(),
                representative: values.get("representative").clone(),
                location: values.get("location").clone(),
                email: values.get("email").clone(),
                telno: values.get("telno").clone(),
                founded_at: founded_at,
                is_public: values.get("is_public").is_some(),
            };

            spawn_local_scoped(async move
            {
                match post_graphql::<CreateGroup>
                (
                    &state,
                    "/account/graphql",
                     create_group::Variables
                     {
                         create_group_input,
                     },
                ).await
                {
                    Ok(data) =>
                    {
                        let upload_group_avatar_input = create_group_additional::UploadGroupAvatarInput
                        {
                            account_name: selected_account.account_name.clone(),
                            group_name: data.create_group.group_name.clone(),
                            file_name: upload_avatar_file_name.get_clone(),
                            base64: upload_avatar_base64.get_clone(),
                            delete_file: false,
                        };

                        let upload_group_cover_input = create_group_additional::UploadGroupCoverInput
                        {
                            account_name: selected_account.account_name.clone(),
                            group_name: data.create_group.group_name.clone(),
                            file_name: upload_cover_file_name.get_clone(),
                            base64: upload_cover_base64.get_clone(),
                            delete_file: false,
                        };

                        post_graphql::<CreateGroupAdditional>
                        (
                            &state,
                            "/account/graphql",
                            create_group_additional::Variables
                            {
                                upload_group_avatar_input,
                                upload_group_cover_input,
                            },
                        ).await.unwrap();
                        let href = format!
                        (
                            "/account/group/{}",
                            data.create_group.group_name
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

    };

    let avatar_file_key = create_signal(Some(props.avatar_file_key.clone()));
    let cover_file_key = create_signal(Some(props.cover_file_key.clone()));

    let advanced_setting = create_signal(false);
    let advanced_setting_block_classes = create_signal("".to_string());
    create_effect(move ||
    {
        let mut classes = "flex flex_column flex_gap_md".to_string();
        if advanced_setting.get() == false
        {
            classes = classes + " hide";
        }
        advanced_setting_block_classes.set(classes);
    });

    view!
    {
        Form
        (
            classes=StrProp("flex flex_column flex_gap_md width_full").into(),
            on_submit=Box::new(save_handler),
        )
        {
            Label(label=t!("pages.account.group.components.group_form.cover.label"))
            {
                GroupCover
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
                                (t!("pages.account.group.components.group_form.cover.button.remove"))
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
                label=t!("pages.account.group.components.group_form.avatar.label"),
                attr:style="align-self: flex-start; width: auto;",
            )
            {
                GroupAvatar
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
                                (t!("pages.account.group.components.group_form.avatar.button.remove"))
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
                label=t!("pages.account.group.components.group_form.group_name.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("group_name").into(),
                    placeholder=StringProp(t!("pages.account.group.components.group_form.group_name.placeholder")).into(),
                    required=BoolProp(true).into(),
                    disabled=BoolProp(props.group_name.is_some()).into(),
                    value=StringProp(props.group_name.unwrap_or_default()).into(),
                )
            }
            Label
            (
                label=t!("pages.account.group.components.group_form.name.label"),
                required=BoolProp(true).into(),
            )
            {
                TextField
                (
                    name=StrProp("name").into(),
                    placeholder=StringProp(t!("pages.account.group.components.group_form.name.placeholder")).into(),
                    required=BoolProp(true).into(),
                    value=StringProp(props.name.unwrap_or_default()).into(),
                )
            }
            Label(label=t!("pages.account.group.components.group_form.description.label"))
            {
                TextField
                (
                    name=StrProp("description").into(),
                    placeholder=StringProp(t!("pages.account.group.components.group_form.description.placeholder")).into(),
                    multiline=BoolProp(true).into(),
                    value=StringProp(props.description.unwrap_or_default()).into(),
                )
            }
            Label(label=t!("pages.account.group.components.group_form.is_public.label"))
            {
                Switch
                (
                    name=StrProp("is_public").into(),
                    checked=BoolProp(props.is_public.unwrap_or_default()).into(),
                )
            }
            Box
            (
                classes=StrProp("flex flex_row flex_align_center flex_gap_md flex_align_self_start clickable").into(),
                on:click=move |_| advanced_setting.set(!advanced_setting.get()),
            )
            {
                (
                    if advanced_setting.get()
                    {
                        view! { Icon(icon=IconKind::Minus.into()) }
                    }
                    else
                    {
                        view! { Icon(icon=IconKind::Plus.into()) }
                    }
                )
                (t!("pages.account.group.components.group_form.advanced_setting"))
            }
            Box(classes=*advanced_setting_block_classes)
            {
                Label(label=t!("pages.account.group.components.group_form.representative.label"))
                {
                    TextField
                    (
                        name=StrProp("representative").into(),
                        placeholder=StringProp(t!("pages.account.group.components.group_form.representative.placeholder")).into(),
                        value=StringProp(props.representative.unwrap_or_default()).into(),
                    )
                }
                Label(label=t!("pages.account.group.components.group_form.location.label"))
                {
                    TextField
                    (
                        name=StrProp("location").into(),
                        placeholder=StringProp(t!("pages.account.group.components.group_form.location.placeholder")).into(),
                        value=StringProp(props.location.unwrap_or_default()).into(),
                    )
                }
                Label(label=t!("pages.account.group.components.group_form.email.label"))
                {
                    TextField
                    (
                        name=StrProp("email").into(),
                        placeholder=StringProp(t!("pages.account.group.components.group_form.email.placeholder")).into(),
                        field_type=StrProp("email").into(),
                        value=StringProp(props.email.unwrap_or_default()).into(),
                    )
                }
                Label(label=t!("pages.account.group.components.group_form.telno.label"))
                {
                    TextField
                    (
                        name=StrProp("telno").into(),
                        placeholder=StringProp(t!("pages.account.group.components.group_form.telno.placeholder")).into(),
                        value=StringProp(props.telno.unwrap_or_default()).into(),
                    )
                }
                Label(label=t!("pages.account.group.components.group_form.founded_at.label"))
                {
                    TextField
                    (
                        name=StrProp("founded_at").into(),
                        field_type=StrProp("date").into(),
                        value=StringProp(props.founded_at.unwrap_or_default()).into(),
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
                    (t!("pages.account.group.components.group_form.button.cancel"))
                }
                Button
                (
                    button_type=StrProp("submit").into(),
                    round=ButtonRound::Full.into(),
                )
                {
                    (
                        if group_name.is_some()
                        {
                            t!("pages.account.group.components.group_form.button.update")
                        }
                        else
                        {
                            t!("pages.account.group.components.group_form.button.send")
                        }
                    )
                }
            }
        }
    }
}
