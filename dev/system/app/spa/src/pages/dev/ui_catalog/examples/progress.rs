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
                Progress(size=ProgressSize::Small.into())
                Progress(size=ProgressSize::Medium.into())
                Progress(size=ProgressSize::Large.into())
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
                Progress(thickness=ProgressThickness::Thin.into())
                Progress(thickness=ProgressThickness::Normal.into())
                Progress(thickness=ProgressThickness::Thick.into())
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
                        Progress(color=color.into())
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
                Progress(variant=ProgressVariant::Linear.into())
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
                            color=color.into(),
                            variant=ProgressVariant::Linear.into(),
                        )
                    }
                )
            }
        }
    }
}
