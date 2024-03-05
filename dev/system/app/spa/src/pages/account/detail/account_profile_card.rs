//------------------------------------------------------------------------------
//! Account profile card.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::props::*;
use crate::utils::request_graphql::post_graphql;

use graphql_client::GraphQLQuery;
use rust_i18n::t;
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;


//------------------------------------------------------------------------------
/// GraphQL.
//------------------------------------------------------------------------------
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema/account.graphql",
    query_path = "graphql/mutation/account.graphql",
    response_derives = "Debug, Clone, PartialEq",
)]
struct SetDefaultAccountProfile;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct AccountProfileCardProps
{
    pub token: String,
    pub account_name: String,
    pub name: String,
    pub bio: String,
    pub affiliation: String,
    pub location: String,
    pub email: String,
    pub telno: String,
    pub birthdate: String,
    pub gender: String,
    pub avatar_file_key: String,
    pub cover_file_key: String,
    pub default_token: Signal<String>,
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn AccountProfileCard<G: Html>( props: AccountProfileCardProps ) -> View<G>
{
    let state: AppState = use_context();
    let edit_href = format!("/account/profile/edit/{}", props.token);

    let cloned_state = state.clone();
    let cloned_account_name = props.account_name.clone();
    let cloned_token = props.token.clone();

    let this_token = props.token.clone();
    let is_default = move || props.default_token.get_clone() == this_token;
    let this_token2 = props.token.clone();
    let is_default2 = move || props.default_token.get_clone() == this_token2;

    view!
    {
        Box(classes=StrProp("padding_bottom_md filled shadow_md radius_md").into())
        {
            Box
            (
                classes=StrProp("margin_bottom_2xl").into(),
                attr:style="position: relative;",
            )
            {
                ProfileCover
                (
                    attr:style="border-radius: var(--radius-md) var(--radius-md) 0 0;",
                    file_key=OptionProp(Some(props.cover_file_key)).into(),
                )
                ProfileAvatar
                (
                    attr:style="
                        position: absolute;
                        bottom: 0;
                        left: 50%;
                        transform: translate(-50%, 50%);
                    ",
                    file_key=OptionProp(Some(props.avatar_file_key)).into(),
                    size=AvatarSize::XXXXL.into(),
                )
                (
                    if is_default()
                    {
                        view!
                        {
                            Chip
                            (
                                variant=ChipVariant::Filled.into(),
                                attr:style="
                                    position: absolute;
                                    top: var(--spacing-sm);
                                    left: var(--spacing-sm);
                                ",
                            )
                            {
                                "default"
                            }
                        }
                    }
                    else
                    {
                        view! {}
                    }
                )
                Tooltip
                (
                    description=view!
                    {
                        (t!("pages.account.profile.account_profile_card.button.edit"))
                    },
                    position=TooltipPosition::Bottom.into(),
                    attr:style="
                        position: absolute;
                        top: var(--spacing-sm);
                        right: var(--spacing-sm);
                    ",
                )
                {
                    FloatingButton
                    (
                        icon=view! { Icon(icon=IconKind::Pen.into()) },
                        href=OptionProp(Some(edit_href)).into(),
                    )
                }
                (
                    if is_default2()
                    {
                        view! {}
                    }
                    else
                    {
                        let cloned_state_inner = cloned_state.clone();
                        let cloned_account_name_inner = cloned_account_name.clone();
                        let cloned_token_inner = cloned_token.clone();
                        let set_default_handler = move |_|
                        {
                            let state = cloned_state_inner.clone();
                            let account_name = cloned_account_name_inner.clone();
                            let account_profile_token = cloned_token_inner.clone();
                            props.default_token.set
                            (
                                account_profile_token.clone()
                            );
                            spawn_local_scoped(async move
                            {
                                let _ = post_graphql::<SetDefaultAccountProfile>
                                (
                                    &state,
                                    "/account/graphql",
                                     set_default_account_profile::Variables
                                     {
                                         account_name,
                                         account_profile_token,
                                     },
                                ).await;
                            });
                        };

                        view!
                        {
                            Tooltip
                            (
                                description=view!
                                {
                                    (t!("pages.account.profile.account_profile_card.button.default"))
                                },
                                position=TooltipPosition::Bottom.into(),
                                attr:style="
                                    position: absolute;
                                    top: var(--spacing-sm);
                                    right: calc(44px + var(--spacing-sm) * 2);
                                ",
                            )
                            {
                                FloatingButton
                                (
                                    icon=view!
                                    {
                                        Icon(icon=IconKind::Person.into())
                                    },
                                    on:click=set_default_handler,
                                )
                            }
                        }
                    }
                )
            }
            Box(classes=StrProp("flex flex_column flex_align_center").into())
            {
                Typography(font_size=TypographyFontSize::XL.into())
                {
                    (props.name)
                }
                Typography(font_weight=TypographyFontWeight::Light.into())
                {
                    "@" (props.account_name)
                }
                Box(classes=StrProp("flex flex_column flex_gap_sm margin_top_lg").into())
                {
                    (
                        if props.affiliation.len() > 0
                        {
                            let cloned_affiliation = props.affiliation.clone();
                            view!
                            {
                                Box(classes=StrProp("flex flex_row flex_align_center flex_gap_sm").into())
                                {
                                    Icon(icon=IconKind::Building.into())
                                    Typography(font_weight=TypographyFontWeight::Light.into())
                                    {
                                        (cloned_affiliation)
                                    }
                                }
                            }
                        }
                        else
                        {
                            view! {}
                        }
                    )
                    (
                        if props.location.len() > 0
                        {
                            let cloned_location = props.location.clone();
                            view!
                            {
                                Box(classes=StrProp("flex flex_row flex_align_center flex_gap_sm").into())
                                {
                                    Icon(icon=IconKind::Location.into())
                                    Typography(font_weight=TypographyFontWeight::Light.into())
                                    {
                                        (cloned_location)
                                    }
                                }
                            }
                        }
                        else
                        {
                            view! {}
                        }
                    )
                    (
                        if props.email.len() > 0
                        {
                            let cloned_email = props.email.clone();
                            view!
                            {
                                Box(classes=StrProp("flex flex_row flex_align_center flex_gap_sm").into())
                                {
                                    Icon(icon=IconKind::Envelope.into())
                                    Typography(font_weight=TypographyFontWeight::Light.into())
                                    {
                                        (cloned_email)
                                    }
                                }
                            }
                        }
                        else
                        {
                            view! {}
                        }
                    )
                    (
                        if props.telno.len() > 0
                        {
                            let cloned_telno = props.telno.clone();
                            view!
                            {
                                Box(classes=StrProp("flex flex_row flex_align_center flex_gap_sm").into())
                                {
                                    Icon(icon=IconKind::Phone.into())
                                    Typography(font_weight=TypographyFontWeight::Light.into())
                                    {
                                        (cloned_telno)
                                    }
                                }
                            }
                        }
                        else
                        {
                            view! {}
                        }
                    )
                    (
                        if props.birthdate.len() > 0
                        {
                            let cloned_birthdate = props.birthdate.clone();
                            view!
                            {
                                Box(classes=StrProp("flex flex_row flex_align_center flex_gap_sm").into())
                                {
                                    Icon(icon=IconKind::Birthday.into())
                                    Typography(font_weight=TypographyFontWeight::Light.into())
                                    {
                                        (cloned_birthdate)
                                    }
                                }
                            }
                        }
                        else
                        {
                            view! {}
                        }
                    )
                    (
                        if props.gender.len() > 0
                        {
                            let cloned_gender = props.gender.clone();
                            view!
                            {
                                Box(classes=StrProp("flex flex_row flex_align_center flex_gap_sm").into())
                                {
                                    Icon(icon=IconKind::Gender.into())
                                    Typography(font_weight=TypographyFontWeight::Light.into())
                                    {
                                        (cloned_gender)
                                    }
                                }
                            }
                        }
                        else
                        {
                            view! {}
                        }
                    )
                }
                (
                    if props.bio.len() > 0
                    {
                        let cloned_bio = props.bio.clone();
                        view!
                        {
                            Divider
                            MultiLineText
                            (
                                classes=StrProp("width_full text_align_center").into(),
                                text=StringProp(cloned_bio).into(),
                            )
                        }
                    }
                    else
                    {
                        view! {}
                    }
                )
            }
        }
    }
}
