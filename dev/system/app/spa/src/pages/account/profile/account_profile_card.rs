//------------------------------------------------------------------------------
//! Account profile card.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct AccountProfileCardProps
{
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
}


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn AccountProfileCard<G: Html>( props: AccountProfileCardProps ) -> View<G>
{
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
