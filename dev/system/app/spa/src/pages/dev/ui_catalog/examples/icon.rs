//------------------------------------------------------------------------------
//! IconExamples.
//------------------------------------------------------------------------------

use crate::components::*;
use crate::layouts::application::MainPanel;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component(inline_props)]
pub fn IconExamples<G: Html>( colors: ReadSignal<Vec<String>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.icon.heading"))
        }
        MainPanel
        {
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon(icon=*create_signal("plus".to_string()))
                Icon(icon=*create_signal("minus".to_string()))
                Icon(icon=*create_signal("caret_right".to_string()))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon
                (
                    icon=*create_signal("plus".to_string()),
                    size=*create_signal("small".to_string()),
                )
                Icon
                (
                    icon=*create_signal("minus".to_string()),
                    size=*create_signal("small".to_string()),
                )
                Icon
                (
                    icon=*create_signal("caret_right".to_string()),
                    size=*create_signal("small".to_string()),
                )
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon(icon=*create_signal("plus".to_string()))
                Icon(icon=*create_signal("minus".to_string()))
                Icon(icon=*create_signal("caret_right".to_string()))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Icon
                (
                    icon=*create_signal("plus".to_string()),
                    size=*create_signal("large".to_string()),
                )
                Icon
                (
                    icon=*create_signal("minus".to_string()),
                    size=*create_signal("large".to_string()),
                )
                Icon
                (
                    icon=*create_signal("caret_right".to_string()),
                    size=*create_signal("large".to_string()),
                )
            }
            Indexed
            (
                iterable=colors,
                view=|color|
                {
                    let cloned_color_1 = color.clone();
                    let cloned_color_2 = color.clone();
                    view!
                    {
                        div(class="flex flex_row flex_gap_md flex_align_center width_full")
                        {
                            Icon
                            (
                                icon=*create_signal("plus".to_string()),
                                color=*create_signal(cloned_color_1),
                            )
                            Icon
                            (
                                icon=*create_signal("minus".to_string()),
                                color=*create_signal(cloned_color_2),
                            )
                            Icon
                            (
                                icon=*create_signal("caret_right".to_string()),
                                color=*create_signal(color),
                            )
                        }
                    }
                }
            )
        }
    }
}
