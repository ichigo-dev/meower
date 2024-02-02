//------------------------------------------------------------------------------
//! TooltipExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn TooltipExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.heading.heading"))
        }
        MainPanel
        {
            div(class="flex flex_row flex_gap_md flex_align_center width_full padding_top_4xl")
            {
                Tooltip(description=view! { "Content" })
                {
                    "Hover me"
                }
                Tooltip
                (
                    description=view! { "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." br() "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." },
                )
                {
                    "Hover me"
                }
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full padding_top_2xl")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Tooltip
                        (
                            color=*create_signal(color),
                            description=view! { "Content" },
                        )
                        {
                            "Hover me"
                        }
                    }
                )
            }
            div(class="flex flex_column flex_gap_md flex_align_center width_full padding_vertical_2xl")
            {
                Tooltip
                (
                    classes=*create_signal("width_6xl text_align_center".to_string()),
                    position=*create_signal("top".to_string()),
                    description=view! { "Content" },
                )
                {
                    "Top"
                }
                div(class="flex flex_row")
                {
                    Tooltip
                    (
                        classes=*create_signal("width_6xl text_align_center".to_string()),
                        position=*create_signal("left".to_string()),
                        description=view! { "Content" },
                    )
                    {
                        "Left"
                    }
                    div(class="width_6xl")
                    Tooltip
                    (
                        classes=*create_signal("width_6xl text_align_center".to_string()),
                        position=*create_signal("right".to_string()),
                        description=view! { "Content" },
                    )
                    {
                        "Right"
                    }
                }
                Tooltip
                (
                    classes=*create_signal("width_6xl text_align_center".to_string()),
                    position=*create_signal("bottom".to_string()),
                    description=view! { "Content" },
                )
                {
                    "Bottom"
                }
            }
        }
    }
}
