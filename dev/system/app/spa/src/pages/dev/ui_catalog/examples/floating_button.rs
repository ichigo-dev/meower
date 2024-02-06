//------------------------------------------------------------------------------
//! FloatingButtonExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;
use crate::utils::props::*;
use crate::variables::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn FloatingButtonExamples<G: Html>
(
    colors: ReadSignal<Vec<Colors>>,
) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.floating_button.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.floating_button.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                FloatingButton(icon=view! { span(class="ui_icon icon_plus") })
                FloatingButton(icon=view! { span(class="ui_icon icon_plus") })
                {
                    "Button"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.floating_button.disabled.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                FloatingButton
                (
                    disabled=BoolProp(true).into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
                FloatingButton
                (
                    disabled=BoolProp(true).into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
                {
                    "Button"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.floating_button.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                FloatingButton
                (
                    size=FloatingButtonSize::Small.into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
                FloatingButton
                (
                    size=FloatingButtonSize::Medium.into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
                FloatingButton
                (
                    size=FloatingButtonSize::Large.into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                FloatingButton
                (
                    size=FloatingButtonSize::Small.into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
                {
                    "Button"
                }
                FloatingButton
                (
                    size=FloatingButtonSize::Medium.into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
                {
                    "Button"
                }
                FloatingButton
                (
                    size=FloatingButtonSize::Large.into(),
                    icon=view! { span(class="ui_icon icon_plus") },
                )
                {
                    "Button"
                }
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.floating_button.sizes.heading"))
            }
            Indexed
            (
                iterable=colors,
                view=|color| view!
                {
                    div(class="flex flex_row flex_gap_md flex_align_center width_full")
                    {
                        FloatingButton
                        (
                            color=color.into(),
                            icon=view! { span(class="ui_icon icon_plus") },
                        )
                        FloatingButton
                        (
                            color=color.into(),
                            icon=view! { span(class="ui_icon icon_plus") },
                        )
                        {
                            "Button"
                        }
                    }
                }
            )
        }
    }
}
