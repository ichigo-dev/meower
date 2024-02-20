//------------------------------------------------------------------------------
//! Account menu button.
//------------------------------------------------------------------------------

use crate::AppState;
use crate::components::*;
use crate::utils::props::*;
use crate::variables::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn AccountMenuButton<G: Html>
(
    node_ref: NodeRef<G>,
    open: Signal<bool>,
) -> View<G>
{
    let state: AppState = use_context();

    view!
    {
        (
            match state.selected_account.get_clone()
            {
                Some(selected_account) =>
                {
                    let file_key = selected_account.avatar_file_key.clone();
                    view!
                    {
                        Box
                        (
                            node_ref=node_ref,
                            classes=StrProp("clickable flex flex_row flex_align_center flex_gap_sm width_full").into(),
                            tag=BoxTag::Button.into(),
                            on:click=move |_| { open.set(!open.get()); },
                        )
                        {
                            Avatar
                            (
                                classes=StrProp("flex_no_shrink").into(),
                                file_key=OptionProp(Some(file_key)).into(),
                                alt=StringProp(t!("common.aside.account_menu_button.avatar.alt")).into(),
                            )
                            Box(classes=StrProp("text_overflow_ellipsis").into())
                            {
                                (selected_account.name)
                            }
                        }
                    }
                },
                None =>
                {
                    view!
                    {
                        Button
                        (
                            color=Colors::Primary.into(),
                        )
                        {
                            (t!("common.aside.account_menu_button.button.create"))
                        }
                    }
                },
            }
        )
    }
}
