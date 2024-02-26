//------------------------------------------------------------------------------
//! Account profile card.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Props.
//------------------------------------------------------------------------------
#[derive(Props)]
pub struct AccountProfileCardProps
{
    pub account_name: String,
    pub name: String,
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
                    attr:style="
                        border-radius: var(--radius-md) var(--radius-md) 0 0;
                    ",
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
                Typography { (props.name) }
                Typography { (props.account_name) }
            }
        }
    }
}
