//------------------------------------------------------------------------------
//! Mini profile.
//------------------------------------------------------------------------------

mod props;

pub use props::MiniProfileProps;

use crate::components::*;
use crate::utils::props::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn MiniProfile<G: Html>( props: MiniProfileProps<G> ) -> View<G>
{
    let classes = move || -> String
    {
        let mut classes = "flex flex_row flex_align_center flex_gap_sm width_full".to_string();
        if props.clickable.get()
        {
            classes.push_str(" clickable");
        }
        classes
    };

    view!
    {
        button
        (
            ref=props.node_ref,
            class=classes(),
            ..props.attributes
        )
        {
            ProfileAvatar
            (
                classes=StrProp("flex_no_shrink").into(),
                file_key=OptionProp(props.file_key).into(),
                alt=StringProp(t!("common.aside.account_menu_button.avatar.alt")).into(),
            )
            Box(classes=StrProp("flex flex_row flex_align_center flex_gap_sm overflow_hidden width_full").into())
            {
                Box(classes=StrProp("flex flex_column overflow_hidden width_full").into())
                {
                    Box(classes=StrProp("text_overflow_ellipsis").into())
                    {
                        (props.name)
                    }
                    Box(classes=StrProp("text_overflow_ellipsis fs_xs").into())
                    {
                        "@"(props.account_name)
                    }
                }
                (
                    if props.selected.get()
                    {
                        view!
                        {
                            Chip(size=ChipSize::Small.into())
                            {
                                "selected"
                            }
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
