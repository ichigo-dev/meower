//------------------------------------------------------------------------------
//! ProgressExamples.
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
pub fn ProgressExamples<G: Html>( colors: ReadSignal<Vec<Colors>> ) -> View<G>
{
    view!
    {
        h3(class="ui_heading h3")
        {
            (t!("pages.dev.ui_catalog.progress.heading"))
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.progress.spin.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Progress
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.progress.spin.sizes.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Progress(size=*create_signal("small".to_string()))
                Progress
                Progress(size=*create_signal("large".to_string()))
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.progress.spin.thickness.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Progress(thickness=*create_signal("thin".to_string()))
                Progress
                Progress(thickness=*create_signal("thick".to_string()))
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.progress.spin.colors.heading"))
            }
            div(class="flex flex_row flex_gap_md flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Progress(color=*create_signal(color))
                    }
                )
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.progress.linear.heading"))
            }
            div(class="flex flex_column flex_gap_md flex_align_center width_full")
            {
                Progress(variant=*create_signal("linear".to_string()))
            }
        }
        MainPanel
        {
            h4(class="ui_heading h4")
            {
                (t!("pages.dev.ui_catalog.progress.linear.colors.heading"))
            }
            div(class="flex flex_column flex_gap_lg flex_align_center width_full")
            {
                Indexed
                (
                    iterable=colors,
                    view=|color| view!
                    {
                        Progress
                        (
                            color=*create_signal(color),
                            variant=*create_signal("linear".to_string()),
                        )
                    }
                )
            }
        }
    }
}
