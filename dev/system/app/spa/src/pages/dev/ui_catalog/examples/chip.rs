//------------------------------------------------------------------------------
//! ChipExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;
use crate::variables::*;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn ChipExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.chip.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.basic.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string())
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.disabled.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    disabled=*create_signal(true),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                    disabled=*create_signal(true),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.clickable.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    clickable=*create_signal(true),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                    clickable=*create_signal(true),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.icon.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    left_icon=view! { span(class="ui_icon icon_envelope") }
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    variant=*create_signal("outlined".to_string()),
                    right_icon=view!
                    {
                        span(class="ui_icon icon_xmark clickable")
                    }
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    size=*create_signal("small".to_string()),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                )
                Chip
                (
                    label=*create_signal("Chip".to_string()),
                    size=*create_signal("large".to_string()),
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.chip.colors.heading"))
            }
            Indexed
            (
                iterable=colors,
                view=|color|
                {
                    let cloned_color = color.clone();
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Chip
                            (
                                label=*create_signal("Chip".to_string()),
                                color=*create_signal(cloned_color),
                            )
                            Chip
                            (
                                label=*create_signal("Chip".to_string()),
                                color=*create_signal(color),
                                variant=*create_signal("outlined".to_string()),
                            )
                        }
                    }
                }
            )
        }
    }
}

