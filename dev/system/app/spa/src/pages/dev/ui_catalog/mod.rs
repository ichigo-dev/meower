//------------------------------------------------------------------------------
//! UI catalog page.
//------------------------------------------------------------------------------

mod examples;
use examples::*;

use crate::layouts::application::Main;
use crate::variables::Colors;

use rust_i18n::t;
use sycamore::prelude::*;


//------------------------------------------------------------------------------
/// Component.
//------------------------------------------------------------------------------
#[component]
pub fn UiCatalog<G: Html>() -> View<G>
{
    let theme_colors = create_signal(vec!
    [
        Colors::Primary,
        Colors::Secondary,
        Colors::Error,
        Colors::Warning,
        Colors::Success,
        Colors::Info,
    ]);

    view!
    {
        Main(heading=t!("pages.dev.ui_catalog.heading_top"))
        {
            BadgeExamples(colors=*theme_colors)
            ChipExamples(colors=*theme_colors)
            HeadingExamples(colors=*theme_colors)
            IconExamples(colors=*theme_colors)
            ListExamples(colors=*theme_colors)
            TableExamples(colors=*theme_colors)
            TooltipExamples(colors=*theme_colors)
            AlertExamples()
            DialogExamples(colors=*theme_colors)
            ProgressExamples(colors=*theme_colors)
            SkeletonExamples()
            SnackbarExamples(colors=*theme_colors)
            ButtonExamples(colors=*theme_colors)
            ButtonGroupExamples(colors=*theme_colors)
            CheckboxExamples(colors=*theme_colors)
            FloatingButtonExamples(colors=*theme_colors)
            RadioExamples(colors=*theme_colors)
            RangeSliderExamples()
        }
    }
}
