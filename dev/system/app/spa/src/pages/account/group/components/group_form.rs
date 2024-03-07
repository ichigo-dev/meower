//------------------------------------------------------------------------------
//! Create group form.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{ Event, FileReader, HtmlInputElement, MouseEvent, ProgressEvent };


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
    };

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
            Label(label=t!("pages.account.group.components.group_form.is_public.label"))
            {
                Switch
                (
                    name=StrProp("is_public").into(),
                    checked=BoolProp(props.is_public.unwrap_or_default()).into(),
                )
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
